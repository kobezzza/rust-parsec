use super::*;

pub fn lookup<P: Parser>(parser: P) -> Lookup<P> {
    Lookup(parser)
}

#[derive(Debug)]
pub struct Lookup<P>(P);

impl<P: Parser> Parser for Lookup<P> {
    type Output = ();

    fn parse<'a>(&self, i: ParserIterator<'a>) -> ParserResult<'a, Self::Output> {
        let _ = self.0.parse(i.clone())?;
        Ok(((), i))
    }
}