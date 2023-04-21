mod plain_text;
mod video_psalm;

use std::fmt::Display;

use serde::Deserialize;

pub use plain_text::PlainText;
pub use video_psalm::video_psalm;

#[derive(Debug)]
pub enum ParserType {
    VideoPsalm,
    PlainText,
    Unknown(String),
}

impl Display for ParserType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VideoPsalm => f.write_str("VideoPsalm"),
            Self::PlainText => f.write_str("PlainText"),
            Self::Unknown(s) => f.write_fmt(format_args!("Unknown: {}", s)),
        }
    }
}

impl<'de> Deserialize<'de> for ParserType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut s = String::deserialize(deserializer)?.to_lowercase();
        s.retain(|c| !matches!(c, ' ' | '-' | '_'));

        Ok(match s.as_str() {
            "videopsalm" => Self::VideoPsalm,
            "plaintext" => Self::PlainText,
            _ => Self::Unknown(s),
        })
    }
}
