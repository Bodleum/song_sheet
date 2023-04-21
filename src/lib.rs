use std::{fs, path::PathBuf};

use config::Config;
use error::SongSheetError;
use log::{info, trace};

use crate::{latex::LaTeX, song::Song};

pub mod config;
pub mod error;
pub mod latex;
pub mod parser;
pub mod song;

pub fn run(config: &Config) -> Result<(), SongSheetError> {
    let tex_path = format!("{}.tex", config.name);
    trace!("Creating new latex file {tex_path}.");
    let mut latex = LaTeX::new(&tex_path)?;

    trace!("Reading {} to string.", &config.source);
    let buf = fs::read_to_string(&config.source).map_err(|source| SongSheetError::ReadError {
        path: PathBuf::from(&config.source),
        source,
    })?;

    info!("Parsing {}.", &config.source);
    let songs: Vec<Song> = parser::video_psalm(&buf.trim_start_matches('\u{feff}'))?
        .into_iter()
        .filter(|s| !config.exclude.contains(&s.title))
        .collect();
    for song in songs {
        trace!("Adding {} to latex.", song.title);
        latex.add_song(song);
    }

    info!("Writing to LaTeX file.");
    let latex = latex.write_to_file()?;

    info!("Compiling LaTeX.");
    latex.compile()?;

    info!("Cleaning up LaTeX files.");
    latex.clean()?;

    info!("Done!");
    Ok(())
}
