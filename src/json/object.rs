use std::collections::HashMap;

use crate::{seq, or};
use crate::json::err::JSONError;
use crate::parsec::*;

use super::json::*;
use super::string::*;

pub fn start() -> impl Parser<Output = JsonValue> {
    fmt(tag("{"), |_, mut i| {
        i.push_state(JsonValue::StartObject);
        Ok((JsonValue::StartObject, i))
    })
}

pub fn next_key() -> impl Parser<Output = JsonValue> {
    fmt(tag(","), |_, i| Ok((JsonValue::ExpectedObjectKey, i)))
}

pub fn next_value() -> impl Parser<Output = JsonValue> {
    fmt(tag(":"), |_, i| Ok((JsonValue::ExpectedObjectValue, i)))
}

pub fn start_end_key() -> impl Parser<Output = JsonValue> {
    fmt(tag("\""), |_, mut i| {
        if i.check_state(&JsonValue::StartObjectKey) {
            i.pop_state();
            return Ok((JsonValue::EndObjectKey, i))
        }

        i.push_state(JsonValue::StartObjectKey);
        Ok((JsonValue::StartObjectKey, i))
    })
}

pub fn key() -> impl Parser<Output = JsonValue> {
    fmt(pass(), |_, i| {
        if !i.check_state(&JsonValue::StartObjectKey) {
            return Err(JSONError::new(i.current_pos()))
        }

        let (output, remaining) = take(|ch| ch != '"', 0..).parse(i)?;

        Ok((JsonValue::ObjectKey(output), remaining))
    })
}

pub fn value() -> impl Parser<Output = JsonValue> {
    fmt(pass(), |_, i| {
        if !i.check_state(&JsonValue::StartObject) {
            return Err(JSONError::new(i.current_pos()))
        }

        let (output, remaining) = rec(json_stream).parse(i)?;

        Ok((JsonValue::ObjectValue(Box::new(output)), remaining))
    })
}

pub fn end() -> impl Parser<Output = JsonValue> {
    fmt(tag("}"), |_, mut i| {
        if !i.check_state(&JsonValue::StartObject) {
            return Err(JSONError::new(i.current_pos()))
        }

        i.pop_state();

        Ok((JsonValue::EndObject, i))
    })
}


pub fn object() -> Box<dyn Parser<Output = JsonValue>> {
    let el = || fmt(seq!(
        trim(string()),
        trim(tag(":")),
        rec(json),
    ), |((key, _), el), i| Ok(((key, el), i)));

    let elems = seq!(
        el(),

        trim(
            or!(
                seq(tag(","), lookup(not(tag("}")))),
                lookup(tag("}"))
            )
        )
    );

    let object = fmt(seq!(
        tag("{"),

        repeat(
            fmt(elems, |(el, _), i| Ok((el, i))),
            0..
        ),

        trim(tag("}")),

    ), |((_, els), ..), i| {
        let mut map = HashMap::with_capacity(els.len());

        for (key, el) in els {
            match key {
                JsonValue::String(key) => map.insert(key, el),
                _ => unreachable!()
            };
        }

        Ok((JsonValue::Object(map), i))
    });

    Box::new(object)
}