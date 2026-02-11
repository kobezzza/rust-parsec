use std::collections::HashMap;

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

pub fn skip_whitespace() -> impl Parser {
    skip(|c| c.is_whitespace())
}

pub fn null() -> impl Parser<Output = JsonValue> {
     fmt(tag("null"), |_| JsonValue::Null)
}

pub fn boolean() -> impl Parser<Output = JsonValue> {
    fmt(or!(tag("true"), tag("false")), |(t, ..)| JsonValue::Bool(t.is_some()))
}

pub fn number() -> impl Parser<Output = JsonValue> {
    let int = || {
        let sign = fmt(or!(tag("-"), tag("+")), |(minus, ..)| {
            if minus.is_some() { "-".to_string() } else { "+".to_string() }
        });

        let int = seq!(
            opt(sign),
            take(|ch| ch.is_digit(10))
        );

        fmt(int, |(sign, int)| {
            let sign = sign.unwrap_or("".to_string());
            let mut s = String::with_capacity(sign.len() + int.len() + 1);

            s.push_str(&sign);
            s.push_str(&int);
            s
        })
    };

    let exp = seq!(or!(tag("e"), tag("E")), int());

    let exp = fmt(exp, |((..), int)| {
        let mut s = String::with_capacity(int.len() + 1);
        s.push('e');
        s.push_str(&int);
        s
    });

    let float = seq!(
        tag("."),
        take(|ch| ch.is_digit(10)),
        opt(exp)
    );

    let float = fmt(float, |((_, int), exp)| {
        let exp = exp.unwrap_or("".to_string());
        let mut s = String::with_capacity(int.len() + exp.len() + 1);

        s.push('.');
        s.push_str(&int);
        s.push_str(&exp);
        s
    });

    let num = seq!(int(), opt(float));

    fmt(num, |(int, float)| {
        let num = int + &float.unwrap_or("".to_string());
        JsonValue::Number(num.parse().unwrap_or(f64::NAN))
    })
}

pub fn string<'a>() -> impl Parser<Output = JsonValue> {
    let parser = seq!(
        tag("\""),
        take(|ch| ch != '"'),
        tag("\""),
    );

    fmt(parser, |((_, str), ..)| JsonValue::String(str))
}

pub fn array() -> Box<dyn Parser<Output = JsonValue>> {
    let empty = fmt(seq!(
        tag("["),
        skip_whitespace(),
        tag("]"),
    ), |_| JsonValue::Array(vec![]));

    let single = fmt(seq!(
        tag("["),
        rec(json),
        tag("]"),
    ), |((_, el), ..)| JsonValue::Array(vec![el]));

    let multiple = fmt(seq!(
        tag("["),

        repeat(
            fmt(seq!(rec(json), tag(",")), |(el, _)| el),
            1..
        ),

        rec(json),
        tag("]"),

    ), |(((_, mut els), el), ..)| JsonValue::Array({ els.push(el); els }));

    Box::new(or_same!(empty, single, multiple))
}

pub fn json() -> impl Parser<Output = JsonValue> {
    or_same!(
        array(),
        null(),
        boolean(),
        string(),
        number(),
    )
}
