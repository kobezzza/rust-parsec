mod err;

use super::*;
use err::NotError;

pub fn not<P: Parser>(parser: P) -> Not<P> {
    Not(parser)
}

#[derive(Debug)]
pub struct Not<P>(P);

impl<P: Parser> Parser for Not<P> {
    type Output = ();

    fn parse<'a>(&self, mut i: ParserIterator<'a>) -> ParserResult<'a, Self::Output> {
        match self.0.parse(i.clone()) {
            Ok((_, i)) => Err(NotError::new(i.current_pos())),
            Err(err) => {
                i.change_pos(err.position());
                Ok(((), i))
            }
        }
    }
}