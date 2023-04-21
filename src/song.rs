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

impl Song {
    pub fn new(name: &str) -> Self {
        Self {
            title: name.to_string(),
            order: String::new(),
            verses: Vec::<String>::new(),
            chorus: None,
            bridge: None,
        }
    }

    pub fn add_verse(&mut self, verse: &str) {
        self.verses.push(verse.to_string());
    }

    pub fn set_chorus(&mut self, chorus: &str) {
        self.chorus = Some(chorus.to_string());
    }

    pub fn set_bridge(&mut self, bridge: &str) {
        self.bridge = Some(bridge.to_string());
    }

    pub fn set_order(&mut self, order: &str) -> Result<(), SongError> {
        // Check chorus
        if order.contains('c') && self.chorus.is_none() {
            return Err(SongError::NoChorus {
                song_title: self.title.clone(),
            });
        }

        // Check bridge
        if order.contains('b') && self.bridge.is_none() {
            return Err(SongError::NoBridge {
                song_title: self.title.clone(),
            });
        }

        // Check there are enough verses
        let v_count = order.chars().filter(|c| *c == 'v').count();
        let v_spec = self.verses.len();
        if v_count < v_spec {
            return Err(SongError::NotEnoughVerses {
                song_title: self.title.clone(),
                expected: v_count,
                actual: v_spec,
            });
        }

        self.order = order.to_string();
        Ok(())
    }
}
