use crate::error::SongError;

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
    pub title: String,
    pub order: String,
    pub verses: Vec<String>,
    pub chorus: Option<String>,
    pub bridge: Option<String>,
}

// Self {
//     title: name.to_string(),
//     order: String::new(),
//     verses: Vec::<String>::new(),
//     chorus: None,
//     bridge: None,
// }

impl Song {
    pub fn builder(title: &str) -> SongBuilder {
        SongBuilder {
            title: title.to_owned(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct SongBuilder {
    title: String,
    order: Option<String>,
    verses: Option<Vec<String>>,
    chorus: Option<String>,
    bridge: Option<String>,
}

impl SongBuilder {
    pub fn add_verse(mut self, verse: &str) -> Self {
        self.verses.get_or_insert(Vec::new()).push(verse.to_owned());
        self
    }

    pub fn set_chorus(mut self, chorus: &str) -> Self {
        self.chorus = Some(chorus.to_owned());
        self
    }

    pub fn set_bridge(mut self, bridge: &str) -> Self {
        self.bridge = Some(bridge.to_owned());
        self
    }

    pub fn set_order(mut self, order: &str) -> Self {
        self.order = Some(order.to_owned());
        self
    }

    pub fn build(self) -> Result<Song, SongError> {
        let order = self.order.ok_or(SongError::NoOrder {
            song_title: self.title.clone(),
        })?;

        // Check chorus
        if order.contains('c') && self.chorus.is_none() {
            return Err(SongError::NoChorus {
                song_title: self.title,
            });
        }

        // Check bridge
        if order.contains('b') && self.bridge.is_none() {
            return Err(SongError::NoBridge {
                song_title: self.title,
            });
        }

        // Check there are enough verses
        let verses = self.verses.ok_or(SongError::NotEnoughVerses {
            song_title: self.title.clone(),
            expected: 1,
            actual: 0,
        })?;
        let v_act = verses.len();
        let v_exp = order.chars().filter(|c| *c == 'v').count();

        if v_exp < v_act {
            return Err(SongError::NotEnoughVerses {
                song_title: self.title,
                expected: v_exp,
                actual: v_act,
            });
        }

        Ok(Song {
            title: self.title,
            order,
            verses,
            chorus: self.chorus,
            bridge: self.bridge,
        })
    }
}
