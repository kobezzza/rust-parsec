mod null;
mod boolean;
mod number;
mod string;
mod array;
mod object;
mod json;
mod err;

use crate::iter::ParserIterator;
use crate::parsec::{Parser, ParserResult};

pub fn parse<'a, T: Into<ParserIterator<'a>>>(i: T) -> ParserResult<'a, json::JsonValue> {
    json::json().parse(i.into())
}

pub fn parse_stream<'a, T: Into<ParserIterator<'a>>>(i: T) -> StreamParser<'a> {
    StreamParser { i: Some(i.into()) }
}

pub struct StreamParser<'a> {
    i: Option<ParserIterator<'a>>,
}

impl<'a> Iterator for StreamParser<'a> {
    type Item = json::JsonValue;

    fn next(&mut self) -> Option<Self::Item> {
        let i = std::mem::replace(&mut self.i, None)?;

        if let Ok((output, remaining)) = json::json_stream().parse(i) {
            self.i = Some(remaining);
            return Some(output)
        }

        None
    }
}