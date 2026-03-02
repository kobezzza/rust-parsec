mod tag; pub use tag::*;
mod take; pub use take::*;
mod trim; pub use trim::*;

mod seq; pub use seq::*;
mod or; pub use or::*;
mod or_same; pub use or_same::*;

mod repeat; pub use repeat::*;
mod opt; pub use opt::*;
mod lookup; pub use lookup::*;
mod not; pub use not::*;

mod rec; pub use rec::*;
mod fmt; pub use fmt::*;

mod err; pub use err::*;

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