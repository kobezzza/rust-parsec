use crate::parsec::*;
use super::json::*;

pub fn null() -> impl Parser<Output = JsonValue> {
    fmt(tag("null"), |_, i| Ok((JsonValue::Null, i)))
}
