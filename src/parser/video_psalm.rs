use std::error::Error;

use log::{info, trace, warn};
use serde::Deserialize;

use crate::Song;

/// Represents a VideoPsalm Song Book
/// ```
/// struct VS { /* private fields */ }
/// ```
#[derive(Deserialize)]
struct VS {
    #[serde(rename = "Songs")]
    songs: Vec<VsSong>,
}

/// Represents a song in a VideoPsalm Song Book
/// ```
/// struct VsSong { /* private fields */ }
/// ```
#[derive(Deserialize)]
struct VsSong {
    // #[serde(rename = "Author")]
    // author: String,
    // #[serde(rename = "Copyright")]
    // copyright: String,
    #[serde(rename = "Text")]
    title: String,
    #[serde(rename = "Verses")]
    stanzas: Vec<VsStanza>,
}

#[derive(Deserialize)]
struct VsStanza {
    /// Text of the stanza
    #[serde(rename = "Text")]
    text: String,

    /// VideoPsalm gives some stanzas a tag. From observation we have:
    ///  - No tag means verse
    ///  - 1 means chorus
    ///  - 3 means bridge
    ///   - 6 means repeat
    #[serde(rename = "Tag")]
    tag: Option<i32>,
}

pub fn video_psalm<T>(input: &T) -> Result<Vec<Song>, Box<dyn Error>>
where
    T: AsRef<str>,
{
    trace!("Parsing as json.");
    let json: VS = serde_json::from_str(input.as_ref())?;

    Ok(json
        .songs
        .iter()
        .map(|j| {
            trace!("Creating new Song for {}.", &j.title);
            let mut s = Song::new(&j.title);
            let mut order = String::new();
            trace!("Iterating over stanzas in {}.", &j.title);
            for stanza in &j.stanzas {
                match stanza.tag {
                    None => {
                        s.add_verse(&stanza.text);
                        order.push('v');
                    }
                    Some(tag) => {
                        if tag == 1 {
                            s.set_chorus(&stanza.text);
                            order.push('c');
                        } else if tag == 3 {
                            s.set_bridge(&stanza.text);
                            order.push('b');
                        } else if tag == 6 {
                            info!("Repeat: Ignoring.\nRepeating: {}.", &stanza.text);
                        } else {
                            warn!("Unknown tag type {}.", tag);
                        }
                    }
                }
            }
            s.set_order(&order)
                .unwrap_or_else(|s| panic!("Unable to set order on {}. {}.", &j.title, s));
            s
        })
        .collect::<Vec<Song>>())
}
