use super::*;

pub fn opt<P: Parser>(parser: P) -> Opt<P> {
    Opt(parser)
}

#[derive(Debug)]
pub struct Opt<P>(P);

impl<P: Parser> Parser for Opt<P> {
    type Output = Option<P::Output>;

    fn parse<'a>(&self, i: ParserIterator<'a>) -> ParserResult<'a, Self::Output> {
        if let Ok((output, remaining)) = self.0.parse(i.clone()) {
            return Ok((Some(output), remaining));
        }

        Ok((None, i))
    }
}