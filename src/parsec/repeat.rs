mod err;

use std::ops::{Range, Bound, RangeBounds};

use super::*;
use crate::iter::ParserIterator;

pub use err::RepeatError;

pub fn repeat<P: Parser>(parser: P, range: impl RangeBounds<usize>) -> Repeat<P> {
    let min = match range.start_bound() {
        Bound::Unbounded => 0,
        Bound::Included(&i) => i,
        Bound::Excluded(&i) => i.saturating_add(1), // ?
    };

    let max = match range.end_bound() {
        Bound::Unbounded => usize::MAX,
        Bound::Included(&i) => i,
        Bound::Excluded(&i) => i - 1
    };

    if min > max {
        panic!("Паттерн повторения задан неверно");
    }

    Repeat { p: parser, range: min..max }
}

#[derive(Debug)]
pub struct Repeat<P> {
    p: P,
    range: Range<usize>
}

impl<P: Parser> Parser for Repeat<P> {
    type Output = Vec<P::Output>;

    fn parse<'a>(&self, mut i: ParserIterator<'a>) -> ParserResult<'a, Self::Output> {
        let mut counter = 0usize;

        let mut result = Vec::new();

        while
            counter < self.range.end &&
            let Ok((output, remaining)) = self.p.parse(i.clone())
        {
            result.push(output);
            i = remaining;
            counter += 1;
        }

        if counter < self.range.start {
            return Err(RepeatError::new(i.current_pos(), "Недостаточное количество повторений парсера"))
        }

        Ok((result, i))
    }
}