use std::collections::HashMap;

use std::error::Error;
use std::fmt::Display;
use std::num::ParseFloatError;

use crate::parsec::*;
use crate::{seq, or, or_same};

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

pub fn null() -> impl Parser<Output = JsonValue> {
     fmt(tag("null"), |_, i| Ok((JsonValue::Null, i)))
}

pub fn boolean() -> impl Parser<Output = JsonValue> {
    fmt(or!(tag("true"), tag("false")), |(t, ..), i| {
        Ok((JsonValue::Bool(t.is_some()), i))
    })
}

pub fn number() -> impl Parser<Output = JsonValue> {
    let int = || {
        let sign = fmt(or!(tag("-"), tag("+")), |(minus, ..), i| {
            Ok((if minus.is_some() { "-".to_string() } else { "+".to_string() }, i))
        });

        let int = seq!(
            opt(sign),
            take(|ch| ch.is_digit(10))
        );

        fmt(int, |(sign, int), i| {
            let sign = sign.unwrap_or("".to_string());

            let mut s = String::with_capacity(sign.len() + int.len() + 1);
            s.push_str(&sign);
            s.push_str(&int);

            Ok((s, i))
        })
    };

    let exp = seq!(or!(tag("e"), tag("E")), int());

    let exp = fmt(exp, |(_, int), i| {
        let mut s = String::with_capacity(int.len() + 1);
        s.push('e');
        s.push_str(&int);

        Ok((s, i))
    });

    let float = seq!(
        tag("."),
        take(|ch| ch.is_digit(10)),
        opt(exp)
    );

    let float = fmt(float, |((_, int), exp), i| {
        let exp = exp.unwrap_or("".to_string());
        let mut s = String::with_capacity(int.len() + exp.len() + 1);

        s.push('.');
        s.push_str(&int);
        s.push_str(&exp);

        Ok((s, i))
    });

    let num = seq!(int(), opt(float));

    fmt(num, |(int, float), i| {
        let num = int + &float.unwrap_or("".to_string());

        #[derive(Debug)]
        pub struct ParseNumError(usize, ParseFloatError);

        impl Display for ParseNumError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.1)
            }
        }

        impl Error for ParseNumError {
            fn source(&self) -> Option<&(dyn Error + 'static)> {
                Some(&self.1)
            }
        }

        impl ParseError for ParseNumError {
            fn position(&self) -> usize {
                self.0
            }
        }

        match num.parse() {
            Ok(num) => Ok((JsonValue::Number(num), i)),
            Err(e) => {
                let e = Box::new(ParseNumError(i.current_pos(), e));
                Err(e)
            },
        }
    })
}

pub fn string<'a>() -> impl Parser<Output = JsonValue> {
    let parser = seq!(
        tag("\""),
        take(|ch| ch != '"'),
        tag("\""),
    );

    fmt(parser, |((_, str), ..), i| Ok((JsonValue::String(str), i)))
}

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

pub fn json() -> impl Parser<Output = JsonValue> {
    let json = or_same!(
        object(),
        array(),
        null(),
        boolean(),
        string(),
        number(),
    );

    trim(json)
}
