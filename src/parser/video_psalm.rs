use std::error::Error;

use log::{info, trace, warn};
use serde::Deserialize;

use crate::Song;

/// Represents a VideoPsalm Song Book
/// ```
/// struct VS { /* private fields */ }
/// ```
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct VS {
    Songs: Vec<VsSong>,
}

/// Represents a song in a VideoPsalm Song Book
/// ```
/// struct VsSong { /* private fields */ }
/// ```
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct VsSong {
    // Author: String,
    // Copyright: String,
    /// Title of the song
    Text: String,
    Verses: Vec<VsStanza>,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct VsStanza {
    /// Text of the stanza
    Text: String,

    /// VideoPsalm gives some stanzas a tag. From observation we have:
    ///  - No tag means verse
    ///  - 1 means chorus
    ///  - 3 means bridge
    ///   - 6 means repeat
    Tag: Option<i32>,
}

pub fn video_psalm<T>(input: &T) -> Result<Vec<Song>, Box<dyn Error>>
where
    T: AsRef<str>,
{
    trace!("Parsing as json.");
    let json: VS = serde_json::from_str(input.as_ref())?;

    Ok(json
        .Songs
        .iter()
        .map(|j| {
            trace!("Creating new Song for {}.", &j.Text);
            let mut s = Song::new(&j.Text);
            let mut order = String::new();
            trace!("Iterating over stanzas in {}.", &j.Text);
            for stanza in &j.Verses {
                match stanza.Tag {
                    None => {
                        s.add_verse(&stanza.Text);
                        order.push('v');
                    }
                    Some(tag) => {
                        if tag == 1 {
                            s.set_chorus(&stanza.Text);
                            order.push('c');
                        } else if tag == 3 {
                            s.set_bridge(&stanza.Text);
                            order.push('b');
                        } else if tag == 6 {
                            info!("Repeat: Ignoring.\nRepeating: {}.", &stanza.Text);
                        } else {
                            warn!("Unknown tag type {}.", tag);
                        }
                    }
                }
            }
            s.set_order(&order)
                .unwrap_or_else(|s| panic!("Unable to set order on {}. {}.", &j.Text, s));
            s
        })
        .collect::<Vec<Song>>())
}
