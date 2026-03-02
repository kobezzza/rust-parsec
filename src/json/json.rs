use std::any::Any;
use std::collections::HashMap;

use crate::parsec::*;
use crate::iter::ParserState;
use crate::{or_same};

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    StartString,
    String(String),
    EndString,

    Null,
    Bool(bool),
    Number(f64),

    StartArray,
    ExpectedArrayElem,
    ArrayElem(Box<JsonValue>),
    EndArray,

    Array(Vec<JsonValue>),

    StartObject,
    ExpectedObjectKey,
    StartObjectKey,
    ObjectKey(String),
    EndObjectKey,
    ExpectedObjectValue,
    StartObjectValue,
    ObjectValue(Box<JsonValue>),
    EndObjectValue,
    EndObject,

    Object(HashMap<String, JsonValue>),
}

impl ParserState for JsonValue {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn ParserState> {
        Box::new(self.clone())
    }
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

pub fn json_stream() -> impl Parser<Output = JsonValue> {
    let json = or_same!(
        object::start_end_key(),
        object::key(),

        string::start_end(),
        string::value(),

        null::null(),
        boolean::boolean(),
        object::key(),
        number::number(),

        array::start(),
        array::end(),
        array::next_elem(),
        array::elem(),

        object::start(),
        object::end(),

        object::next_key(),
        object::next_value(),
        object::value(),
    );

    trim(json)
}