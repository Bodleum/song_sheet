use std::{collections::HashSet, fs, path::Path};

use serde::Deserialize;

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
}
use config_defaults::*;

use crate::error::ConfigError;

impl Config {
    pub fn read<P>(path: &P) -> Result<Self, ConfigError>
    where
        P: AsRef<Path> + ?Sized,
    {
        let s = fs::read_to_string(path).map_err(|source| ConfigError::ReadError {
            path: path.as_ref().to_path_buf(),
            source,
        })?;
        let c: Config = toml::from_str(&s)?;
        Ok(c)
    }
}
