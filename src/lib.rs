pub mod config;
pub mod latex;
pub mod parser;

/// Represents a Song
#[derive(Debug, Default)]
pub struct Song {
    pub name: String,
    pub order: String,
    pub verses: Vec<String>,
    pub chorus: Option<String>,
    pub bridge: Option<String>,
}

impl Song {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            order: String::new(),
            verses: Vec::<String>::new(),
            chorus: None,
            bridge: None,
        }
    }

    pub fn add_verse(&mut self, verse: &str) {
        self.verses.push(
            verse
                .lines()
                .map(|l| format!("    {{{}}}", l))
                .collect::<Vec<String>>()
                .join("\n"),
        );
    }

    pub fn set_chorus(&mut self, chorus: &str) {
        self.chorus = Some(
            chorus
                .lines()
                .map(|l| format!("    {{{}}}", l))
                .collect::<Vec<String>>()
                .join("\n"),
        );
    }

    pub fn set_bridge(&mut self, bridge: &str) {
        self.bridge = Some(
            bridge
                .lines()
                .map(|l| format!("    {{{}}}", l))
                .collect::<Vec<String>>()
                .join("\n"),
        );
    }

    pub fn set_order(&mut self, order: &str) -> Result<(), &str> {
        // Check chorus
        if order.contains('c') && self.chorus == None {
            return Err("ERROR: No chorus specified!");
        }

        // Check bridge
        if order.contains('b') && self.bridge == None {
            return Err("ERROR: No bridge specified!");
        }

        // Check there are enough verses
        if order.chars().filter(|c| *c == 'v').count() < self.verses.len() {
            return Err("ERROR: Not enough verses!");
        }

        self.order = order.to_string();
        Ok(())
    }
}
