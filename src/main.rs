use crate::parsec::*;

mod parsec;
mod iter;
mod json;

fn main() {
    let mut a = json::parse_stream(r#"{"a": [1, 2, 3]}"#);

    for i in a {
        println!("{:?}", i);
    }
}
