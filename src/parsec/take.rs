use super::*;

pub fn take<P: Fn(char) -> bool>(pred: P) -> Take<P> {
    Take { pred }
}

#[derive(Debug)]
pub struct Take<P> {
    pred: P
}

impl<P: Fn(char) -> bool> Parser for Take<P> {
    type Output = String;

    fn parse<'a>(&self, mut i: ParserIterator<'a>) -> ParserResult<'a, Self::Output> {
        let mut result = String::new();

        while let Some(ch) = i.peek() {
            if (self.pred)(ch) {
                result.push(ch);
                i.next();

            } else {
                break;
            }
        }

        Ok((result, i))
    }
}
