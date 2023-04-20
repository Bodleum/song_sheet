use std::fs;

use config::Config;
use errors::AppError;
use log::{info, trace};

use crate::latex::LaTeX;

pub mod config;
pub mod errors;
pub mod latex;
pub mod parser;

/// Represents the type of stanza
#[derive(Debug, Default)]
pub enum StanzaType {
    #[default]
    Verse,
    Chorus,
    Bridge,
}

/// Represents a Song
#[derive(Debug, Default)]
pub struct Song {
    pub name: String,
    pub order: String,
    pub verses: Vec<String>,
    pub chorus: Option<String>,
    pub bridge: Option<String>,
}

impl Song {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            order: String::new(),
            verses: Vec::<String>::new(),
            chorus: None,
            bridge: None,
        }
    }

    pub fn add_verse(&mut self, verse: &str) {
        self.verses.push(verse.to_string());
    }

    pub fn set_chorus(&mut self, chorus: &str) {
        self.chorus = Some(chorus.to_string());
    }

    pub fn set_bridge(&mut self, bridge: &str) {
        self.bridge = Some(bridge.to_string());
    }

    pub fn set_order(&mut self, order: &str) -> Result<(), String> {
        // Check chorus
        if order.contains('c') && self.chorus.is_none() {
            return Err(String::from(
                "ERROR: Order calls for a chorus, but song has none specified!",
            ));
        }

        // Check bridge
        if order.contains('b') && self.bridge.is_none() {
            return Err(String::from(
                "ERROR: Order calls for bridge, but song has none specified!",
            ));
        }

        // Check there are enough verses
        let v_count = order.chars().filter(|c| *c == 'v').count();
        let v_spec = self.verses.len();
        if v_count < v_spec {
            return Err(format!(
                "ERROR: Order calls for {} verses, but only {} specified!",
                v_count, v_spec
            ));
        }

        self.order = order.to_string();
        Ok(())
    }
}

pub fn run(config: &Config) -> error_stack::Result<(), AppError> {
    let tex_path = format!("{}.tex", config.name);
    trace!("Creating new latex file {tex_path}.");
    let mut latex = LaTeX::new(&tex_path)?;

    trace!("Reading {} to string.", &config.source);
    let buf = fs::read_to_string(&config.source).unwrap();

    info!("Parsing {}.", &config.source);
    let songs: Vec<Song> = parser::video_psalm(&buf.trim_start_matches('\u{feff}'))
        .unwrap()
        .into_iter()
        .filter(|s| !config.exclude.contains(&s.name))
        .collect();
    for song in songs {
        trace!("Adding {} to latex.", song.name);
        latex.add_song(song);
    }

    info!("Writing to LaTeX file.");
    let latex = latex.write_to_file()?;

    info!("Compiling LaTeX.");
    latex.compile(&tex_path).unwrap();

    info!("Cleaning up LaTeX files.");
    latex.clean().unwrap();

    info!("Done!");
    Ok(())
}
