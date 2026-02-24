mod err;

use std::ops::{Range, Bound, RangeBounds};

use super::*;
use err::TakeError;

pub fn take<P: Fn(char) -> bool>(pred: P, range: impl RangeBounds<usize>) -> Take<P> {
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

    Take { pred, range: min..max }
}

#[derive(Debug)]
pub struct Take<P> {
    pred: P,
    range: Range<usize>
}

impl<P: Fn(char) -> bool> Parser for Take<P> {
    type Output = String;

    fn parse<'a>(&self, mut i: ParserIterator<'a>) -> ParserResult<'a, Self::Output> {
        let mut counter = 0usize;

        let mut result = String::new();

        while
            counter < self.range.end &&
            let Some(ch) = i.peek()
        {
            if (self.pred)(ch) {
                result.push(ch);
                i.next();
                counter += 1;

            } else {
                break;
            }
        }

        if counter < self.range.start {
            return Err(TakeError::new(i.current_pos(), "Недостаточное количество повторений"))
        }

        Ok((result, i))
    }
}
