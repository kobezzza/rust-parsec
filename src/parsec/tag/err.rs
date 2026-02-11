use std::fmt::Display;
use std::error::Error;
use crate::parsec::ParseError;

#[derive(Debug)]
pub struct TagError(usize, String);

impl TagError {
    pub fn new(byte_pos: usize, msg: impl Into<String>) -> Box<Self> {
        Box::new(TagError(byte_pos, msg.into()))
    }
}

impl Display for TagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.1)
    }
}

impl Error for TagError {}

impl ParseError for TagError {
    fn position(&self) -> usize {
        self.0
    }
}
