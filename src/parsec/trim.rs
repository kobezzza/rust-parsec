mod err;

use crate::parsec::trim::err::TrimError;
use super::*;

pub fn trim<P: Parser>(parser: P) -> Trim<P> {
    Trim(parser)
}

#[derive(Debug)]
pub struct Trim<P>(P);

impl<P: Parser> Parser for Trim<P> {
    type Output = P::Output;

    fn parse<'a>(&self, mut i: ParserIterator<'a>) -> ParserResult<'a, Self::Output> {
        while let Some(ch) = i.peek() {
            if ch.is_whitespace() {
                i.next();
                continue;
            }

            return self.0.parse(i);
        }

        Err(TrimError::new(i.current_pos()))
    }
}
