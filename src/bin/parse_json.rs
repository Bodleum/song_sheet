#![allow(non_snake_case, dead_code)]

use std::{error::Error, fs};

use log::{debug, trace};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct BA {
    Songs: Vec<Song>,
}

#[derive(Debug, Deserialize)]
struct Song {
    Author: String,
    Copyright: String,
    Text: String,
    Verses: Vec<Stanza>,
}

#[derive(Debug, Deserialize)]
struct Stanza {
    Text: String,
    Tag: Option<i32>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Init logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("trace"));

    let path = r#"VS/BonAccordHymns.json"#;
    trace!("Opening {path}.");
    let file = fs::read_to_string(path).expect("Error reading file.");

    trace!("Reading as JSON.");
    let json: BA = serde_json::from_str(file.trim_start_matches('\u{feff}')).unwrap();
    trace!("Successful!");
    debug!("{:#?}", json);

    Ok(())
}

fn untyped_example() -> Result<(), serde_json::Error> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);
    dbg!(v);

    Ok(())
}
