use std::collections::HashMap;

use crate::parsec::*;
use crate::{or_same};

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

pub fn json() -> impl Parser<Output = JsonValue> {
    let json = or_same!(
        null::null(),
        boolean::boolean(),
        string::string(),
        number::number(),
        object::object(),
        array::array(),
    );

    trim(json)
}