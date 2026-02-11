use super::*;

pub fn skip<P: Fn(char) -> bool>(pred: P) -> Skip<P> {
    Skip { pred }
}

#[derive(Debug)]
pub struct Skip<P> {
    pred: P
}

impl<P: Fn(char) -> bool> Parser for Skip<P> {
    type Output = ();

    fn parse<'a>(&self, mut i: ParserIterator<'a>) -> ParserResult<'a, Self::Output> {
        while let Some(ch) = i.peek() {
            if (self.pred)(ch) {
                i.next();

            } else {
                break;
            }
        }

        Ok(((), i))
    }
}
