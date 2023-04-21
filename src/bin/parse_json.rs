#![allow(dead_code)]
use std::fs;

use anyhow::{Context, Result};
use log::{debug, trace};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct BA {
    #[serde(rename = "Songs")]
    songs: Vec<Song>,
}

#[derive(Debug, Deserialize)]
struct Song {
    #[serde(rename = "Author")]
    author: String,
    #[serde(rename = "Copyright")]
    copyright: String,
    #[serde(rename = "Text")]
    title: String,
    #[serde(rename = "Verses")]
    verses: Vec<Stanza>,
}

#[derive(Debug, Deserialize)]
struct Stanza {
    #[serde(rename = "Text")]
    text: String,
    #[serde(rename = "Tag")]
    tag: Option<i32>,
}

fn main() -> Result<()> {
    // Init logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("trace"));

    let path = r#"VS/BonAccordHymns.json"#;
    trace!("Opening {path}.");
    let file = fs::read_to_string(path).with_context(|| "Error reading file.")?;

    trace!("Reading as JSON.");
    let json: BA = serde_json::from_str(file.trim_start_matches('\u{feff}'))
        .with_context(|| "Error parsing JSON.")?;
    trace!("Successful!");
    debug!("{:#?}", json);

    Ok(())
}
