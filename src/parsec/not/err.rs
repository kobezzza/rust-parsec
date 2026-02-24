use std::fmt::Display;
use std::error::Error;
use crate::parsec::ParseError;

#[derive(Debug)]
pub struct NotError(usize);

impl NotError {
    pub fn new(byte_pos: usize) -> Box<Self> {
        Box::new(NotError(byte_pos))
    }
}

impl Display for NotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "Успешное срабатывание вложенного парсера")
    }
}

impl Error for NotError {}

impl ParseError for NotError {
    fn position(&self) -> usize {
        self.0
    }
}
