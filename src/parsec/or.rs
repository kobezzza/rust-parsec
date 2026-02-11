use super::*;
use crate::parsec::Parser;

#[macro_export]
macro_rules! or {
    ($parser:expr) => {
        $parser
    };

    ($first:expr, $second:expr $(,)?) => {
        or($first, $second)
    };

    ($first:expr, $second:expr, $($rest:expr),+ $(,)?) => {
        or!(
            or($first, $second),
            $($rest),+
        )
    };
}

pub fn or<A: Parser, B: Parser>(a: A, b: B) -> Or<A, B> {
    Or(a, b)
}

#[derive(Debug)]
pub struct Or<A, B>(A, B);

impl<A: Parser, B: Parser> Parser for Or<A, B> {
    type Output = (Option<A::Output>, Option<B::Output>);

    fn parse<'a>(&self, i: ParserIterator<'a>) -> ParserResult<'a, Self::Output> {
        match self.0.parse(i.clone()) {
            Ok((output, remaining)) => {
                let output = (Some(output), None);
                Ok((output, remaining))
            }

            _ => {
                let (output, remaining) = self.1.parse(i)?;
                let output = (None, Some(output));
                Ok((output, remaining))
            },
        }
    }
}