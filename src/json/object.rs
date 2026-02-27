use std::collections::HashMap;

use crate::{seq, or};
use crate::parsec::*;

use super::json::*;
use super::string::*;

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