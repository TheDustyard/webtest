use askama::Template;
use dlopen::wrapper::{Container};
use std::fmt::Display;
use thiserror::Error;

use plugin_api::PluginAPI;

fn main() {
    // println!(
    //     "{}",
    //     MyCoolMessage {
    //         name: "jeff",
    //         year: "1998",
    //         country: "london"
    //     }
    // );
    /*
    eprintln!(
        "{}",
        ErrorMessage {
            appname: "pee hahahahaah",
            error: format!("{}", DataStoreError::Redaction("Bruh".into()))
        }
    );

    eprintln!(
        "{}",
        ErrorMessage {
            appname: "pee hahahahaah",
            error: format!(
                "{}",
                DataStoreError::InvalidHeader {
                    expected: "bruh".into(),
                    found: "bru".into()
                }
            )
        }
    ); */

    for file in std::fs::read_dir("plugins").unwrap() {
        let file = file.unwrap();

        println!("{:?}:", file.path());

        let cont: Container<PluginAPI> =
            match unsafe { Container::load(file.path().display().to_string()) } {
                Err(e) => {
                    eprintln!("\t{}", e);
                    continue;
                }
                Ok(cont) => cont,
            };

        cont.init();

        println!(
            "\treply(\"joe mamma\") = {}\n\trun(\"7\") = {}\n\trun(\"17\") = {}",
            cont.reply("joe mamma"),
            cont.run("7"),
            cont.run("17")
        )
    }
}

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] std::io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

#[derive(Template)]
#[template(path = "error.txt")]
struct ErrorMessage<'a, T: Display> {
    appname: &'a str,
    error: T,
}
