use crate::{seq, or};
use crate::json::err::JSONError;
use crate::parsec::*;
use super::json::*;

pub fn start() -> impl Parser<Output = JsonValue> {
    fmt(tag("["), |_, mut i| {
        i.push_state(JsonValue::StartArray);
        Ok((JsonValue::StartArray, i))
    })
}

pub fn next_elem() -> impl Parser<Output = JsonValue> {
    fmt(tag(","), |_, i| Ok((JsonValue::ExpectedArrayElem, i)))
}

pub fn elem() -> impl Parser<Output = JsonValue> {
    fmt(pass(), |_, i| {
        if !i.check_state(&JsonValue::StartArray) {
            return Err(JSONError::new(i.current_pos()))
        }

        let (output, remaining) = rec(json_stream).parse(i)?;

        Ok((JsonValue::ArrayElem(Box::new(output)), remaining))
    })
}

pub fn end() -> impl Parser<Output = JsonValue> {
    fmt(tag("]"), |_, mut i| {
        if !i.check_state(&JsonValue::StartArray) {
            return Err(JSONError::new(i.current_pos()))
        }

        i.pop_state();

        Ok((JsonValue::EndArray, i))
    })
}

pub fn array() -> Box<dyn Parser<Output = JsonValue>> {
    let elem = seq!(
        rec(json),

        trim(
            or!(
                seq(tag(","), lookup(not(trim(tag("]"))))),
                lookup(tag("]"))
            )
        )
    );

    let array = fmt(seq!(
        tag("["),
        repeat(fmt(elem, |(el, _), i| Ok((el, i))), 0..),
        trim(tag("]")),

    ), |((_, els), ..), i| Ok((JsonValue::Array(els), i)));

    Box::new(array)
}