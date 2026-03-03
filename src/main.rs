use crate::parsec::*;

mod parsec;
mod iter;
mod json;

fn main() {
    let mut a = json::parse_stream(r#"{"a":"#);

    for i in a.by_ref() {
        println!("{:?}", i);
    }

    a.push_data(r#"42}"#);

    for i in a.by_ref() {
        println!("{:?}", i);
    }
}
