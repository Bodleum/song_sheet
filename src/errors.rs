use std::{error::Error, fmt};

#[derive(Debug)]
pub struct FileError;
impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Error reading file.")
    }
}
impl Error for FileError {}

#[derive(Debug)]
pub struct DirError;
impl fmt::Display for DirError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Error reading file.")
    }
}
impl Error for DirError {}
