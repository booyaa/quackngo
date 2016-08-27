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
    // let mut res = client.get("http://api.duckduckgo.\
    //                           com/?q=valley+forge+national+park&format=json&pretty=1")
    //                     .send()
    //                     .unwrap();

    // panics - category
    let mut res = client.get("http://api.duckduckgo.\
                              com/?q=simpsons+characters&format=json&pretty=1")
                        .send()
                        .unwrap();

    // panics - disamibiguation
    // let mut res = client.get("http://api.duckduckgo.com/?q=apple&format=json&pretty=1")
    //                     .send()
    //                     .unwrap();

    let mut buffer = String::new();
    res.read_to_string(&mut buffer).expect("no body");
    println!("{}", buffer);

    let instant_answer: InstantAnswer = serde_json::from_str(&buffer).unwrap();
    println!("{:#?}", instant_answer);
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
