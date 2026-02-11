use super::*;
use crate::parsec::Parser;

#[macro_export]
macro_rules! seq {
    ($parser:expr) => {
        $parser
    };

    ($first:expr, $second:expr $(,)?) => {
        seq($first, $second)
    };

    ($first:expr, $second:expr, $($rest:expr),+ $(,)?) => {
        seq!(
            seq($first, $second),
            $($rest),+
        )
    };
}

pub fn seq<A: Parser, B: Parser>(a: A, b: B) -> Seq<A, B> {
    Seq(a, b)
}

#[derive(Debug)]
pub struct Seq<A, B>(A, B);

impl<A: Parser, B: Parser> Parser for Seq<A, B> {
    type Output = (A::Output, B::Output);

    fn parse<'a>(&self, i: ParserIterator<'a>) -> ParserResult<'a, Self::Output> {
        let (output_a, remaining) = self.0.parse(i)?;
        let (output_b, remaining) = self.1.parse(remaining)?;
        Ok(((output_a, output_b), remaining))
    }
}