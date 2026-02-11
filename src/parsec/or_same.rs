use super::*;
use crate::parsec::Parser;

#[macro_export]
macro_rules! or_same {
    ($parser:expr) => {
        $parser
    };

    ($first:expr, $second:expr $(,)?) => {
        or_same($first, $second)
    };

    ($first:expr, $second:expr, $($rest:expr),+ $(,)?) => {
        or_same!(
            or_same($first, $second),
            $($rest),+
        )
    };
}

pub fn or_same<A, B, O>(a: A, b: B) -> OrSame<A, B>
where
    A: Parser<Output = O>,
    B: Parser<Output = O>,
    O: Debug,
{
    OrSame(a, b)
}

#[derive(Debug)]
pub struct OrSame<A, B>(A, B);

impl<A, B, O: Debug> Parser for OrSame<A, B>
where
    A: Parser<Output = O>,
    B: Parser<Output = O>,
    O: Debug,
{
    type Output = O;

    fn parse<'a>(&self, i: ParserIterator<'a>) -> ParserResult<'a, Self::Output> {
        match self.0.parse(i.clone()) {
            Ok(res) => return Ok(res),
            Err(_) => {}
        }

        self.1.parse(i)
    }
}