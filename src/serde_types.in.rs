// Copyright 2016 Mark Sta Ana.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, at your option.
// This file may not be copied, modified, or distributed except
// according to those terms.

use serde::{de, Deserializer};

#[derive(Serialize, Deserialize, Debug)]
/// DuckDuckGo InstantAnswer
pub struct InstantAnswer {
    #[serde(rename = "Abstract")]
    pub abstract_rich: String,
    #[serde(rename = "AbstractText")]
    /// topic summary (with no HTML)
    pub abstract_text: String,
    #[serde(rename = "AbstractSource")]
    /// name of Abstract source
    pub abstract_source: String,
    #[serde(rename = "AbstractURL")]
    /// deep link to expanded topic page in AbstractSource
    pub abstract_url: String,
    #[serde(rename = "Image")]
    /// link to image that goes with Abstract
    pub image: String,
    #[serde(rename = "Heading")]
    ///  name of topic that goes with Abstract
    pub heading: String,
    #[serde(rename = "Answer")]
    /// instant answer
    pub answer: String,
    #[serde(rename = "Redirect")]
    /// !bang redirect URL
    pub redirect: String,
    #[serde(rename = "AnswerType")]
    /// type of Answer, e.g. calc, color, digest, info, ip, iploc, phone, pw, rand, regexp, unicode, upc, or zip
    pub answer_type: String,
    #[serde(rename = "Definition")]
    /// dictionary definition (may differ from Abstract)
    pub definition: String,
    #[serde(rename = "DefinitionSource")]
    /// name of Definition source
    pub definition_source: String,
    #[serde(rename = "DefinitionURL")]
    /// deep link to expanded definition page in DefinitionSource
    pub definition_url: String,
    #[serde(rename = "RelatedTopics")]
    /// `Vec<RelatedTopics>` of internal links to related topics associated with Abstract
    pub related_topics: Vec<RelatedTopic>,
    #[serde(rename = "Results")]
    pub results: Vec<RelatedTopic>,
    #[serde(rename = "Type")]
    /// response category, i.e. A (article), D (disambiguation), C (category), N (name), E (exclusive), or nothing.
    pub response_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// child struct of InstantAnswer
pub struct RelatedTopic {
    #[serde(rename = "Result")]
    pub result: String,
    #[serde(rename = "Icon")]
    pub icon: Icon,
    #[serde(rename = "FirstURL")]
    pub first_url: String,
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// child struct of InstantAnswer
pub struct Topic {
    #[serde(rename = "Result")]
    pub result: String,
    #[serde(rename = "Icon")]
    pub icon: Icon,
    #[serde(rename = "FirstURL")]
    pub first_url: String,
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// child struct of InstantAnswer
pub struct Icon {
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "Height",deserialize_with="deserialize_u64_or_empty_string")]
    pub height: u64,
    #[serde(rename = "Width",deserialize_with="deserialize_u64_or_empty_string")]
    pub width: u64,
}


/// Custom Deserializer
/// Strategies for handling null RelatedTopics (doesn't appear to be a problem in v0.8 serde)
/// - http://stackoverflow.com/questions/37870428/convert-two-types-into-a-single-type-with-serde
/// - http://stackoverflow.com/questions/38037235/handling-mixed-object-arrays-in-serde
/// Source: http://stackoverflow.com/a/37871403/105282
struct DeserializeU64OrEmptyStringVisitor;
impl de::Visitor for DeserializeU64OrEmptyStringVisitor {
    type Value = u64;

    fn visit_u64<E>(&mut self, v: u64) -> Result<Self::Value, E>
        where E: de::Error
    {
        Ok(v)
    }

    fn visit_str<E>(&mut self, v: &str) -> Result<Self::Value, E>
        where E: de::Error
    {
        if v == "" {
            Ok(0)
        } else {
            Err(E::invalid_value("Got a non-empty string")) // is worth trying to parse?
        }
    }
}

fn deserialize_u64_or_empty_string<D>(deserializer: &mut D) -> Result<u64, D::Error>
    where D: Deserializer
{
    deserializer.deserialize(DeserializeU64OrEmptyStringVisitor)
}
