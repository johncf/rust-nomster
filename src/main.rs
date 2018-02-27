#[macro_use]
extern crate nom;

mod parser;

fn main() {
    let (next, consumed) = parser::start("Hello, world!\n<hr>\n<page=1>").unwrap();
    println!("consumed={:?}, next={:?}", consumed, next);
}
