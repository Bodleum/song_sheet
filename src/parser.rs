use std::fs::File;

pub struct Parser {
    file: File,
}

impl Parser {
    pub fn new(file: File) -> Parser {
        Parser { file }
    }

    pub fn create(path: &str) -> Result<Parser, std::io::Error> {
        let file = File::create(path)?;
        Ok(Parser::new(file))
    }
}
