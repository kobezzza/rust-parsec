use super::*;
use crate::parsec::Parser;

pub fn fmt<P, F, O>(p: P, fmt: F) -> Fmt<P, F>
where
    P: Parser,
    F: Fn(P::Output) -> O,
    O: Debug,
{
    Fmt { p, fmt }
}

#[derive(Debug)]
pub struct Fmt<P, F> {
    p: P,
    fmt: F,
}

impl<P, F, O> Parser for Fmt<P, F>
where
    P: Parser,
    F: Fn(P::Output) -> O,
    O: Debug,
{
    type Output = O;

    fn parse<'a>(&self, i: ParserIterator<'a>) -> ParserResult<'a, Self::Output> {
        let (output, remaining) = self.p.parse(i)?;
        Ok(((self.fmt)(output), remaining))
    }
}