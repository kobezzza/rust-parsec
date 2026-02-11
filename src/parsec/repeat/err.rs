use std::fmt::Display;
use std::error::Error;
use crate::parsec::ParseError;

#[derive(Debug)]
pub struct RepeatError(usize, String);

impl RepeatError {
    pub fn new(byte_pos: usize, msg: impl Into<String>) -> Box<Self> {
        Box::new(RepeatError(byte_pos, msg.into()))
    }
}

impl Display for RepeatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.1)
    }
}

impl Error for RepeatError {}

impl ParseError for RepeatError {
    fn position(&self) -> usize {
        self.0
    }
}
