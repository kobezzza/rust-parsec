use super::*;
use crate::parsec::Parser;

pub fn rec<F, P>(f: F) -> Rec<F> where
    F: Fn() -> P,
    P: Parser,
{
    Rec(f)
}

pub struct Rec<F>(F);

impl<F, P> Parser for Rec<F>
where
    F: Fn() -> P,
    P: Parser
{
    type Output = P::Output;

    fn parse<'a>(&self, i: ParserIterator<'a>) -> ParserResult<'a, Self::Output> {
        self.0().parse(i)
    }
}