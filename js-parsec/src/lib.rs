mod utils;

use rust_parsec::json::*;

use js_sys::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

#[wasm_bindgen(start)]
pub fn init() {
    utils::set_panic_hook();
}

#[wasm_bindgen(js_name = newStr)]
pub fn new_str(str: Vec<u8>) -> Vec<usize> {
    let len = str.len();
    vec![str.leak().as_ptr() as usize, len]
}

#[wasm_bindgen(js_name = freeBuffer)]
pub fn free_buffer(ptr: *mut u8, len: usize) {
    let _: Vec<u8> = unsafe { Vec::from_raw_parts(ptr, len, len) };
}

#[wasm_bindgen]
pub struct JsonParser {
    s: &'static str,
    p: StreamParser<'static>,
}

#[wasm_bindgen]
impl JsonParser {
    #[wasm_bindgen(constructor)]
    pub fn new(ptr: *mut u8, len: usize) -> Self {
        let str: &str = unsafe { std::mem::transmute(std::slice::from_raw_parts(ptr, len)) };
        Self { s: str, p: parse_stream(str) }
    }

    pub fn next(&mut self) -> JsValue {
        fn pack(value: impl Into<JsValue>, t: &str, done: bool) -> JsValue {
            let obj = Object::new();
            let value_box = Object::new();

            Reflect::set(
                &value_box,
                &"value".into(),
                &value.into(),
            ).unwrap();

            Reflect::set(
                &value_box,
                &"type".into(),
                &t.into(),
            ).unwrap();

            Reflect::set(
                &obj,
                &JsValue::from("value"),
                &value_box,
            ).unwrap();

            Reflect::set(
                &obj,
                &JsValue::from("done"),
                &done.into(),
            ).unwrap();

            obj.into()
        }

        match self.p.next() {
            Some(Ok(JsonValue::Null)) => pack(JsValue::null(), "null", false),
            Some(Ok(JsonValue::Bool(o))) => pack(o, "boolean", false),
            Some(Ok(JsonValue::Number(o))) => pack(o, "number", false),

            Some(Ok(JsonValue::StartString)) => self.next(),
            Some(Ok(JsonValue::String(o))) => pack(o, "string", false),
            Some(Ok(JsonValue::EndString)) => self.next(),

            Some(Ok(JsonValue::StartArray)) => pack("[", "array_start", false),
            Some(Ok(JsonValue::ExpectedArrayElem)) => self.next(),
            Some(Ok(JsonValue::EndArray)) => pack("]", "array_end", false),

            _ => pack(false, "pass", false),
        }
    }
}

#[wasm_bindgen]
pub fn parse(ptr: *mut u8, len: usize) -> JsonParser {
    JsonParser::new(ptr, len)
}
