use std::fs;

use config::Config;
use error::{ParseError, SongSheetError};
use log::{info, trace};

use crate::{latex::LaTeX, parser::ParserType::*, song::Song};

pub mod config;
pub mod error;
pub mod latex;
pub mod parser;
pub mod song;

pub fn run(config: &Config) -> Result<(), SongSheetError> {
    let mut latex_builder = LaTeX::builder_default(config)?;

    let songs = parse_source(config)?;
    for song in songs {
        trace!("Adding {} to latex.", song.title);
        latex_builder = latex_builder.add_song(song);
    }

    info!("Writing to LaTeX file.");
    let latex = latex_builder.write_to_file()?;

    info!("Compiling LaTeX.");
    latex.compile()?;

    info!("Cleaning up LaTeX files.");
    latex.clean()?;

    info!("Done!");
    Ok(())
}

fn parse_source(config: &Config) -> Result<Vec<Song>, ParseError> {
    match &config.from {
        PlainText => Err(ParseError::NotAvailable(PlainText)),
        VideoPsalm => video_psalm_songs(config),
        Unknown(s) => Err(ParseError::UnknownParser(s.clone())),
    }
}

fn video_psalm_songs(config: &Config) -> Result<Vec<Song>, ParseError> {
    trace!("Reading {} to string.", &config.source);
    let buf = fs::read_to_string(&config.source).map_err(|source| ParseError::ReadError {
        path: config.name.clone(),
        source,
    })?;

    info!("Parsing {}.", &config.source);
    Ok(parser::video_psalm(&buf.trim_start_matches('\u{feff}'))?
        .into_iter()
        .filter(|s| !config.exclude.contains(&s.title))
        .collect())
}
