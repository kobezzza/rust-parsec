use crate::seq;
use crate::parsec::*;
use super::json::*;

pub fn string<'a>() -> impl Parser<Output = JsonValue> {
    let parser = seq!(
        tag("\""),
        take(|ch| ch != '"', 0..),
        tag("\""),
    );

    fmt(parser, |((_, str), ..), i| Ok((JsonValue::String(str), i)))
}