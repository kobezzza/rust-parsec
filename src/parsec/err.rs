use std::fmt::Display;
use std::error::Error;
use crate::parsec::ParseError;

#[derive(Debug)]
pub struct DataError(usize);

impl DataError {
    pub fn new(byte_pos: usize) -> Box<Self> {
        Box::new(DataError(byte_pos))
    }
}

impl Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Ожидаются новые данные")
    }
}

impl Error for DataError {}

impl ParseError for DataError {
    fn position(&self) -> usize {
        self.0
    }
}
