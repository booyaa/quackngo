

// strategies for handling null RelatedTopics (doesn't appear to be a problem in v0.8 serde)
// http://stackoverflow.com/questions/37870428/convert-two-types-into-a-single-type-with-serde
// http://stackoverflow.com/questions/38037235/handling-mixed-object-arrays-in-serde
#[derive(Serialize, Deserialize, Debug)]
struct InstantAnswer {
    #[serde(rename = "Abstract")]
    abstract_rich: String,
    #[serde(rename = "AbstractText")]
    abstract_text: String,
    #[serde(rename = "AbstractSource")]
    abstract_source: String,
    #[serde(rename = "AbstractURL")]
    abstract_url: String,
    #[serde(rename = "Image")]
    image: String,
    #[serde(rename = "Heading")]
    heading: String,
    #[serde(rename = "Answer")]
    answer: String,
    #[serde(rename = "Redirect")]
    redirect: String,
    #[serde(rename = "AnswerType")]
    answer_type: String,
    #[serde(rename = "Definition")]
    definition: String,
    #[serde(rename = "DefinitionSource")]
    definition_source: String,
    #[serde(rename = "DefinitionURL")]
    definition_url: String,
    #[serde(rename = "RelatedTopics")]
    related_topics: Vec<RelatedTopic>,
    #[serde(rename = "Results")]
    results: Vec<RelatedTopic>,
    #[serde(rename = "Type")]
    response_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RelatedTopic {
    #[serde(rename = "Result")]
    result: String,
    #[serde(rename = "Icon")]
    icon: Icon,
    #[serde(rename = "FirstURL")]
    first_url: String,
    #[serde(rename = "Text")]
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Icon {
    #[serde(rename = "URL")]
    url: String,
    // #[serde(rename = "Height")]
    #[serde(rename = "Height",deserialize_with="deserialize_u64_or_empty_string")]
    height: u64,
    #[serde(rename = "Width",deserialize_with="deserialize_u64_or_empty_string")]
    width: u64,
}

use serde::{de, Deserializer};

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
            Err(E::invalid_value("got a non-empty string"))
        }
    }
}

fn deserialize_u64_or_empty_string<D>(deserializer: &mut D) -> Result<u64, D::Error>
    where D: Deserializer
{
    deserializer.deserialize(DeserializeU64OrEmptyStringVisitor)
}
