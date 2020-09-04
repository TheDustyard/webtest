#[macro_use]
extern crate dlopen_derive;

use askama::Template;
use dlopen::wrapper::{WrapperApi};

#[derive(WrapperApi)]
pub struct PluginAPI<'a> {
    init: fn(),
    run: fn(input: &'a str) -> String,
    reply: fn(input: &'a str) -> MyCoolMessage,
}

#[derive(Template)]
#[template(path = "message.txt")]
pub struct MyCoolMessage<'a> {
    pub name: &'a str,
    pub year: &'a str,
    pub country: &'a str,
}