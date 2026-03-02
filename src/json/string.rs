use crate::seq;
use crate::json::err::JSONError;
use crate::parsec::*;
use super::json::*;

pub fn start_end() -> impl Parser<Output = JsonValue> {
    fmt(tag("\""), |_, mut i| {
        if i.check_state(&JsonValue::StartString) {
            i.pop_state();
            return Ok((JsonValue::EndString, i))
        }

        i.push_state(JsonValue::StartString);
        Ok((JsonValue::StartString, i))
    })
}

pub fn value() -> impl Parser<Output = JsonValue> {
    fmt(pass(), |_, i| {
        if !i.check_state(&JsonValue::StartString) {
            return Err(JSONError::new(i.current_pos()))
        }

        let (output, remaining) = take(|ch, escaped| !escaped && ch != '"', 0..)
            .parse(i)?;

        Ok((JsonValue::String(output), remaining))
    })
}

pub fn string() -> impl Parser<Output = JsonValue> {
    let parser = seq!(
        tag("\""),
        take(|ch, escaped| !escaped && ch != '"', 0..),
        tag("\""),
    );

    fmt(parser, |((_, str), ..), i| Ok((JsonValue::String(str), i)))
}