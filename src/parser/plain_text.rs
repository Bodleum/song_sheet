use colored::Colorize;
use nom::{
    bytes::complete::{tag, take_until},
    error::Error,
    Finish, IResult,
};

use crate::song::{Song, StanzaType};
// Parser types

pub struct PlainText;
impl PlainText {
    pub fn parse(input: &str) -> Result<Song, String> {
        let (mut input, name) = match Self::match_name(input) {
            Ok(ok) => ok,
            Err(_) => {
                return Err(format!(
                    "{}: Invalid song title.
Song title must be separated by a blank line.",
                    "SONG ERROR".red().bold()
                ))
            }
        };

        let mut song: Song = Song::new(name);
        let mut order: String = String::new();

        // Get all stanzas
        while !input.is_empty() {
            // Determine stanza type
            let (i, stanza_type) = match Self::start_tag(input).finish() {
                Ok((i, _)) => match Self::identify_tag(i) {
                    Ok(ok) => ok,
                    Err(_) => return Err(format!("{}: Invalid tag.", "SONG ERROR".red().bold())),
                },
                Err(e) => (e.input, StanzaType::Verse),
            };

            // Get stanza
            let (i, stanza) = Self::match_block(i);

            // Add to song
            match stanza_type {
                StanzaType::Verse => {
                    song.add_verse(stanza);
                    order.push('v');
                }
                StanzaType::Chorus => {
                    song.set_chorus(stanza);
                    order.push('c');
                }
                StanzaType::Bridge => {
                    song.set_bridge(stanza);
                    order.push('b');
                }
            }
            input = i;
        }

        song.set_order(&order).unwrap(); // Will never fail if read from file
        Ok(song)
    }

    /// Matches song title, separated by a blank line
    fn match_name(input: &str) -> IResult<&str, &str> {
        let (input, name) = take_until("\n")(input)?;
        let (input, _) = tag("\n\n")(input)?;
        Ok((input, name))
    }

    /// Matches the start of a tag
    fn start_tag(input: &str) -> IResult<&str, &str> {
        tag("#")(input)
    }

    /// Determines stanza type
    fn identify_tag(input: &str) -> IResult<&str, StanzaType> {
        let (input, t) = nom::bytes::complete::take(1usize)(input)?;
        let (input, _) = tag("\n")(input)?;
        match t {
            "c" => Ok((input, StanzaType::Chorus)),
            "b" => Ok((input, StanzaType::Bridge)),
            _ => Err(nom::Err::Error(Error::new(
                input,
                nom::error::ErrorKind::Tag,
            ))),
        }
    }

    /// Matches a stanza, separated by a blank line
    fn match_block(input: &str) -> (&str, &str) {
        match take_until::<&str, &str, Error<&str>>("\n\n")(input) {
            Ok((input, stanza)) => {
                let (input, _) = tag::<&str, &str, Error<&str>>("\n\n")(input).unwrap();
                (input, stanza)
            }
            Err(_) => ("", input),
        }
    }
}
