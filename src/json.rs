mod null;
mod boolean;
mod number;
mod string;
mod array;
mod object;
mod json; pub use json::JsonValue;
mod err;

use std::collections::VecDeque;
use crate::iter::ParserIterator;
use crate::parsec::{ParseError, Parser, ParserResult};

pub fn parse<'a, T: Into<ParserIterator<'a>>>(i: T) -> ParserResult<'a, JsonValue> {
    json::json().parse(i.into())
}

pub fn parse_stream<'a, T: Into<ParserIterator<'a>>>(i: T) -> StreamParser<'a> {
    let mut store = VecDeque::new();
    store.push_back(i.into());
    StreamParser { store: Some(store) }
}

pub struct StreamParser<'a> {
    store: Option<VecDeque<ParserIterator<'a>>>,
}

impl<'a> StreamParser<'a> {
    pub fn push_data<T: Into<ParserIterator<'a>>>(&mut self, data: T) {
        self.store.as_mut().map(|i| i.push_back(data.into()));
    }
}

impl<'a> Iterator for StreamParser<'a> {
    type Item = Result<JsonValue, Box<dyn ParseError>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut store = std::mem::replace(&mut self.store, None)?;

        let mut head = store.pop_front()?;

        if head.is_at_end() {
            if store.is_empty() {
                store.push_front(head);
                self.store = Some(store);
                return None;
            }

            store.front_mut().map(|iter| {
                iter.replace_state(head.take_state());
            });

            self.store = Some(store);
            return self.next();
        }

        match json::json_stream().parse(head) {
            Ok((output, remaining)) => {
                store.push_front(remaining);
                self.store = Some(store);
                Some(Ok(output))
            },

            Err(e) => {
                Some(Err(e))
            }
        }
    }
}