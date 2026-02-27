mod null;
mod boolean;
mod number;
mod string;
mod array;
mod object;
mod json;

use crate::iter::ParserIterator;
use crate::parsec::Parser;

pub fn parse<'a, T: Into<ParserIterator<'a>>>(i: T) -> StreamParser<'a> {
    StreamParser { i: i.into() }
}

pub struct StreamParser<'a> {
    i: ParserIterator<'a>
}

impl<'a> Iterator for StreamParser<'a> {
    type Item = json::JsonValue;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok((output, remaining)) = json::json().parse(self.i.clone()) {
            self.i = remaining;
            return Some(output)
        }

        None
    }
}