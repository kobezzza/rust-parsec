use std::fmt::Display;
use std::error::Error;
use crate::parsec::ParseError;

#[derive(Debug)]
pub struct TakeError(usize, String);

impl TakeError {
    pub fn new(byte_pos: usize, msg: impl Into<String>) -> Box<Self> {
        Box::new(TakeError(byte_pos, msg.into()))
    }
}

impl Display for TakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.1)
    }
}

impl Error for TakeError {}

impl ParseError for TakeError {
    fn position(&self) -> usize {
        self.0
    }
}
