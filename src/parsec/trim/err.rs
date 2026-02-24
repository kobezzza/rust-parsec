use std::fmt::Display;
use std::error::Error;
use crate::parsec::ParseError;

#[derive(Debug)]
pub struct TrimError(usize);

impl TrimError {
    pub fn new(byte_pos: usize) -> Box<Self> {
        Box::new(TrimError(byte_pos))
    }
}

impl Display for TrimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Подходящих данных для парсера не найдено")
    }
}

impl Error for TrimError {}

impl ParseError for TrimError {
    fn position(&self) -> usize {
        self.0
    }
}
