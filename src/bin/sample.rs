extern crate nomster;

#[macro_use]
extern crate structopt;

use nomster::parser;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "FILE", help = "webster file (utf-8)", parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let contents = nomster::read_file(&opt.input).unwrap();
    let mut contents = &*contents;
    while let Some((_, entry, next)) = parser::next_entry2(contents) {
        println!("{:?}", entry);
        contents = next;
    }
}
