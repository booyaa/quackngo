//! A [Duck Duck Go](https://duckduckgo.com) [InstantAnswer API](https://duckduckgo.com/api) library written in rust.
//!
//! Copyright 2016 Mark Sta Ana.
//!
//! Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//! http://www.apache.org/licenses/LICENSE-2.0> at your option. This file may not
//! be copied, modified, or distributed except according to those terms.
//!
//! # Usage
//!
//! This crate is on [crates.io](https://crates.io/crates/quackngo) and can be
//! used by adding `quack` to the dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! quackngo = "0.1"
//! ```
//!
//! and this to your crate root:
//!
//! ```
//! extern crate quackngo;
//! ```
//!
//! ```rust,no_run
//! use quackngo::Quack;
//! println!("{:#?}", Quack::new("!imdb+rushmore"));
//! ```

#![cfg_attr(feature = "serde_macros", feature(plugin, custom_derive))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

extern crate serde;
extern crate serde_json;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate url;

#[cfg(feature = "serde_macros")]
include!("serde_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

use hyper::Client;
use std::io::Read;
use url::percent_encoding;

/// Main struct
pub struct Quack;

const LIB_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const LIB_NAME: &'static str = env!("CARGO_PKG_NAME");
const BASE_URL: &'static str = "http://api.duckduckgo.com/";

impl Quack {
    #[allow(dead_code)]
    #[allow(unused_variables)]
    /// Creates a new Quack client
    ///
    /// Until there's a need the following options are hard coded:
    ///
    /// - format - json, until there's a need to return raw responses
    /// - pretty - pretty printing format is off
    /// - no_redirect - is set, it doesn't make sense to make the http client follow an
    /// redirection given by duckduckgo.
    /// - no_html - is set, until there's a need to markup.
    /// - skip_disambig - is set, this is from my own lack of ability to work out how to have a
    /// collection of mixed types see issue #1
    pub fn new(query: &str) -> InstantAnswer {
        let client = Client::new();

        // let mut url = String::from(BASE_URL);
        // url.push_str("?q=");
        // url.push_str(&query); // TODO: urlencode
        // url.push_str("&format=json");
        // url.push_str("&pretty=0");
        // url.push_str("&no_redirect=1");
        // url.push_str("&skip_disambig=1");
        // url.push_str("&no_html=1");
        // url.push_str("&t=");
        // url.push_str(LIB_NAME);
        // url.push_str("-v");
        // url.push_str(LIB_VERSION);

        let url = build_url(&query);

        debug!("url: {}", &url);

        let mut res = client.get(&url)
                            .send()
                            .unwrap();

        let mut buffer = String::new();
        res.read_to_string(&mut buffer).expect("no body");
        debug!("buffer: {}", buffer);

        serde_json::from_str(&buffer).unwrap()
    }
}

fn build_url(query: &str) -> String {
    let mut url = String::from(BASE_URL);
    url.push_str("?q=");

    let query_encoded = format!("{}",
                percent_encoding::percent_encode(&query.as_bytes(),
                                                 percent_encoding::QUERY_ENCODE_SET));

    url.push_str(&query_encoded);

    url.push_str("&format=json");
    url.push_str("&pretty=0");
    url.push_str("&no_redirect=1");
    url.push_str("&skip_disambig=1");
    url.push_str("&no_html=1");
    url.push_str("&t=");
    url.push_str(LIB_NAME);
    url.push_str("-v");
    url.push_str(LIB_VERSION);

    url.to_string()
}

#[cfg(test)]
/// test cases using curl --silent url > test_case.json
/// topic summaries: http://api.duckduckgo.com/?q=valley+forge+national+park&format=json&pretty=1
/// categories - http://api.duckduckgo.com/?q=simpsons+characters&format=json&pretty=1
/// disamibiguation - http://api.duckduckgo.com/?q=apple&format=json&pretty=1
/// !bang redirect (exclusive) n.b. no_redirect otherwise you'll get a 303 - http://api.duckduckgo.com/?q=!imdb+rushmore&format=json&pretty=1&no_redirect=1

fn harness(test_file: &str) -> InstantAnswer {
    use std::path::PathBuf;
    let mut path = PathBuf::new();
    path.push("tests");
    path.push("fixtures");
    path.push(&test_file);

    let file_path = path.as_os_str();

    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(&file_path).unwrap();

    let mut file_string = String::new();
    file.read_to_string(&mut file_string).unwrap();

    serde_json::from_str(&file_string).unwrap()
}

#[test]
fn should_be_type_exclusive() {
    let result = harness("bang_no_redirect.json");
    assert_eq!("E", result.response_type);
}

#[test]
fn should_be_type_article() {
    let result = harness("topic_summaries.json");
    assert_eq!("A", result.response_type);
}

#[test]
fn should_be_type_category() {
    let result = harness("categories.json");
    assert_eq!("C", result.response_type);
}

// FIXME: fn should_be_type_name() // what kinda query do we need to invoke to see one of these?
// a possible fixes for an array of mixed types http://stackoverflow.com/a/38038277/105282
// http://stackoverflow.com/questions/37561593/rust-serde-json-array-with-different-objects-errors
// most likely solution https://users.rust-lang.org/t/deserializing-enum-json-field-with-serde-with-multiple-types/5424
// #[test]
// FIXME: fn should_be_type_disambiguation() {
//     let result = harness("disambiguation.json");
// println!("{:#?}", result);
//     assert_eq!("D", result.response_type);
// }
