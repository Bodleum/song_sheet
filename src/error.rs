use std::io;

use thiserror::Error;

/// Represents any kind of error which can occur.
#[derive(Debug, Error)]
#[error("An error occured.")]
pub enum SongSheetError {
    /// Represents an error which can occur while obtaining the configuration
    ConfigError(#[from] ConfigError),

    /// Represents an error relating to LaTeX
    LaTeXError(#[from] LaTeXError),

    /// Represents an error while parsing sources
    ParseError(#[from] ParseError),

    /// Represents an error reading from input.
    #[error(r#"Could not read "{}"."#, .path)]
    ReadError { path: String, source: io::Error },
}

/// Represents an error which can occur while obtaining the configuration
#[derive(Debug, Error)]
#[error("An error occured while reading config.")]
pub enum ConfigError {
    /// Represents an error while reading from input.
    #[error(r#"Could not read "{}"."#, .path)]
    ReadError { path: String, source: io::Error },

    /// Represents an error from de-serializing toml
    #[error("Error while de-serializing toml.")]
    TOMLError(#[from] toml::de::Error),
}

/// Represents an error relating to LaTeX
#[derive(Debug, Error)]
#[error("An error occured while generating LaTeX.")]
pub enum LaTeXError {
    /// Represents an error to create a file
    #[error(r#"Could not create file "{}"."#, .path)]
    CreateFileError { path: String, source: io::Error },

    /// Represents an arbitrary I.O. error.
    #[error("I.O. Error.")]
    IOError(#[from] std::io::Error),

    /// Represents an error while adding a package
    #[error("Error adding LaTeX package.")]
    PackageError(String),

    /// Represents an error trying to get a verse which doesn't exist
    #[error("Verse out of bounds. Tried to get verse {index} but there are only {size}.")]
    VerseOutOfBounds { index: usize, size: usize },
}

/// Represents an error while parsing sources
#[derive(Debug, Error)]
#[error("An error occured while parsing sources.")]
pub enum ParseError {
    /// Represents an error while parsing JSON.
    #[error("Error while parsing JSON.")]
    JSONError(#[from] serde_json::Error),

    /// Represents an error setting the order of stanzas in a song.
    #[error("Error setting stanza order: invalid order.")]
    OrderError(#[from] SongError),
}

/// Represents an error relating to a particular song
#[derive(Debug, Error)]
#[error("An error occured while constructing a song.")]
pub enum SongError {
    /// When no order for verses and choruses is specified for a song
    #[error(r#""{song_title}" has no order specified."#)]
    NoOrder { song_title: String },

    /// When a song calls for a chorus but has none specified.
    #[error(r#"Order calls for a chorus, but none specified for "{song_title}"."#)]
    NoChorus { song_title: String },

    /// When a song calls for a bridge but has none specified.
    #[error(r#"Order calls for a bridge, but none specified for "{song_title}"."#)]
    NoBridge { song_title: String },

    /// When there are not enough verses in a song.
    #[error(
        r#"Order calls for {expected} verses, but only {actual} specified for "{song_title}"."#
    )]
    NotEnoughVerses {
        song_title: String,
        expected: usize,
        actual: usize,
    },
}
