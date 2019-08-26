#[macro_use]
extern crate neon;
extern crate regex;

use neon::prelude::*;
use regex::Regex;
use std::fmt::Debug::fmt;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    println!("{:?}", cx);
    Ok(cx.string("hello node"))
}

// fn fetch_emails(mut cx: FunctionContext) -> JsResult<JsArray> {
//     let re = Regex::new(r"(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$)").unwrap();
//     assert!(re.is_match(""))
// }

register_module!(mut cx, {
    cx.export_function("hello", hello)
});
