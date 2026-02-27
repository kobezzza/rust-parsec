use crate::parsec::*;

mod parsec;
mod iter;
mod json;

fn main() {
    let mut a = json::parse(r#"{"a": {"b": null}}"#);

    println!("{:?}", a.next());
}
