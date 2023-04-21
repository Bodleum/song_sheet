use std::{collections::HashSet, fs, path::Path};

use crate::error::ConfigError;
use config_defaults::*;
use serde::Deserialize;

/// Represents the configuration state of the program.
/// ```
/// struct Config {
///    // Options for LaTeX
///    pub keep_tex_file: bool,
///    pub name: String,
///    pub latex_cmd: String,
///    pub latex_args: Vec<String>,
///
///    // Song sheet options
///    pub cover_image: Option<String>,
///    pub chords: bool,
///    // List of song titles to exclude
///    pub exclude: HashSet<String>,
///
///    // Other options
///    pub source: String,
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct Config {
    // Options for LaTeX
    #[serde(default)]
    pub keep_tex_file: bool,
    #[serde(default = "default_name")]
    pub name: String,
    #[serde(default = "default_latex_cmd")]
    pub latex_cmd: String,
    #[serde(default = "default_latex_args")]
    pub latex_args: Vec<String>,

    // Song sheet options
    #[serde(default = "default_cover_image")]
    pub cover_image: String,
    #[serde(default)]
    pub chords: bool,
    // List of song titles to exclude
    #[serde(default)]
    pub exclude: HashSet<String>,

    // Other options
    pub source: String,
}

#[rustfmt::skip]
mod config_defaults {
    pub fn default_name() -> String { String::from("SongSheet") }
    pub fn default_latex_cmd() -> String { String::from("latexmk") }
    pub fn default_latex_args() -> Vec<String> {
        vec![
            String::from("-pdflua"),
            String::from("-interaction=nonstopmode"),
        ]
    }
    pub fn default_cover_image() -> String { String::from("cover_image.jpg") }
}

impl Config {
    pub fn read<P>(path: &P) -> Result<Self, ConfigError>
    where
        P: AsRef<Path> + ?Sized,
    {
        let s = fs::read_to_string(path).map_err(|source| ConfigError::ReadError {
            path: path.as_ref().display().to_string(),
            source,
        })?;
        let c: Config = toml::from_str(&s)?;
        Ok(c)
    }
}
