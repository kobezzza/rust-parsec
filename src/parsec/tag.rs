mod err;

use super::*;

pub use err::TagError;

pub fn tag<T: AsRef<str>>(tag: T) -> Tag<T> {
    Tag { tag }
}

#[derive(Debug)]
pub struct Tag<T> {
    tag: T
}

impl<T: AsRef<str>> Parser for Tag<T> {
    type Output = String;

    fn parse<'a>(&self, mut i: ParserIterator<'a>) -> ParserResult<'a, Self::Output> {
        let tag = self.tag.as_ref();

        for t in tag.chars() {
            match i.peek() {
                Some(ch) => {
                    if ch == t {
                        i.next();

                    } else {
                        let msg = format!("Символ в строке {} не соответствует ожидаемому символу {} тега", i.current_pos(), t);
                        return Err(TagError::new(i.current_pos(), msg));
                    }
                }

                _ => {
                    return Err(TagError::new(i.current_pos(), "Строка закончилась"))
                }
            }
        }

        Ok((tag.to_string(), i))
    }
}