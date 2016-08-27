#![cfg_attr(feature = "serde_macros", feature(plugin, custom_derive))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

extern crate serde;
extern crate serde_json;
extern crate hyper;

#[cfg(feature = "serde_macros")]
include!("serde_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

use hyper::Client;
use std::io::Read;

fn main() {
    let client = Client::new();
    // ok - bang
    // let mut res = client.get("http://api.duckduckgo.\
    //                           com/?q=!imdb+rushmore&format=json&pretty=1&no_redirect=1")
    //                     .send()
    //                     .unwrap();

    // ok - topic
    let mut res = client.get("http://api.duckduckgo.\
                              com/?q=valley+forge+national+park&format=json&pretty=1")
                        .send()
                        .unwrap();

    // panics - category
    // let mut res = client.get("http://api.duckduckgo.\
    //                           com/?q=simpsons+characters&format=json&pretty=1")
    //                     .send()
    //                     .unwrap();

    // panics - disamibiguation
    // let mut res = client.get("http://api.duckduckgo.com/?q=apple&format=json&pretty=1")
    //                     .send()
    //                     .unwrap();

    let mut buffer = String::new();
    res.read_to_string(&mut buffer).expect("no body");
    println!("{}", buffer);

    let instant_answer: InstantAnswer = serde_json::from_str(&buffer).unwrap();
    println!("{:#?}", instant_answer);
    println!("Hello, I do nothing!");
}

// test cases
// topic summaries
// http://api.duckduckgo.com/?q=valley+forge+national+park&format=json&pretty=1
// HTTP/1.1 200 OK
// Server: nginx
// Date: Sat, 27 Aug 2016 10:18:38 GMT
// Content-Type: application/x-javascript
// Connection: keep-alive
// Access-Control-Allow-Origin: *
// X-DuckDuckGo-Results: 1
// Expires: Sat, 27 Aug 2016 10:18:39 GMT
// Cache-Control: max-age=1
// X-DuckDuckGo-Locale: en_US
// categories - http://api.duckduckgo.com/?q=simpsons+characters&format=json&pretty=1
// disamibiguation - http://api.duckduckgo.com/?q=apple&format=json&pretty=1
// !bang redirect n.b. no_redirect otherwise you'll get a 303 - http://api.duckduckgo.com/?q=!imdb+rushmore&format=json&pretty=1&no_redirect=1

#[allow(dead_code)]
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

    // let instant_answer: InstantAnswer =
    serde_json::from_str(&file_string).unwrap()
    // file_string
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

#[test]
fn should_be_type_disambiguation() {
    let result = harness("disambiguation.json");
    assert_eq!("D", result.response_type);
}
