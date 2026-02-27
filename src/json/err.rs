use std::fmt::Display;
use std::error::Error;
use crate::parsec::ParseError;

#[derive(Debug)]
pub struct JSONError(usize);

impl JSONError {
    pub fn new(byte_pos: usize) -> Box<Self> {
        Box::new(JSONError(byte_pos))
    }
}

impl Display for JSONError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Ошибка при парсинге строки")
    }
}

impl Error for JSONError {}

impl ParseError for JSONError {
    fn position(&self) -> usize {
        self.0
    }
}
