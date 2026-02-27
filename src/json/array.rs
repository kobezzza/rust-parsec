use crate::{seq, or};
use crate::parsec::*;
use super::json::*;

pub fn array() -> Box<dyn Parser<Output = JsonValue>> {
    let elems = seq!(
        rec(json),

        trim(
            or!(
                seq(tag(","), lookup(not(tag("]")))),
                lookup(tag("]"))
            )
        )
    );

    let array = fmt(seq!(
        tag("["),
        repeat(fmt(elems, |(el, _), i| Ok((el, i))), 0..),
        trim(tag("]")),

    ), |((_, els), ..), i| Ok((JsonValue::Array(els), i)));

    Box::new(array)
}