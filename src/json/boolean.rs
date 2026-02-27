use crate::or;
use crate::parsec::*;
use super::json::*;

pub fn boolean() -> impl Parser<Output = JsonValue> {
    fmt(or!(tag("true"), tag("false")), |(t, ..), i| {
        Ok((JsonValue::Bool(t.is_some()), i))
    })
}
