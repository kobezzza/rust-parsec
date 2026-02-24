mod tag;
mod seq;
mod or;
mod or_same;
mod take;
mod trim;
mod repeat;
mod opt;
mod rec;
mod fmt;

pub use tag::*;
pub use seq::*;
pub use or::*;
pub use or_same::*;
pub use take::*;
pub use trim::*;
pub use repeat::*;
pub use opt::*;
pub use rec::*;
pub use fmt::*;

use crate::iter::ParserIterator;

use std::error::Error;
use std::fmt::Debug;

pub trait ParseError: Error {
    fn position(&self) -> usize;
}

pub type ParserResult<'a, Output> = Result<(Output, ParserIterator<'a>), Box<dyn ParseError>>;

pub trait Parser {
    type Output: Debug;

    fn parse<'a>(&self, i: ParserIterator<'a>) -> ParserResult<'a, Self::Output>;
}

impl<O: Debug> Parser for Box<dyn Parser<Output = O>> {
    type Output = O;

    fn parse<'a>(&self, i: ParserIterator<'a>) -> ParserResult<'a, Self::Output> {
        (**self).parse(i)
    }
}