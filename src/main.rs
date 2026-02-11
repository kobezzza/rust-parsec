use crate::parsec::*;

mod parsec;
mod iter;
mod json;

fn main() {
    let a = json::json().parse("[     ]".into());

    println!("{:?}", a);
}
