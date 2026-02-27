use std::fmt::Display;
use std::error::Error;
use std::num::ParseFloatError;

use crate::{or, seq};
use crate::parsec::*;

use super::json::*;

pub fn number() -> impl Parser<Output = JsonValue> {
    let int = || {
        let sign = fmt(or!(tag("-"), tag("+")), |(minus, ..), i| {
            Ok((if minus.is_some() { "-".to_string() } else { "+".to_string() }, i))
        });

        let int = seq!(
            opt(sign),
            take(|ch| ch.is_digit(10), 1..)
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
        take(|ch| ch.is_digit(10), 0..),
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