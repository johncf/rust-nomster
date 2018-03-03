extern crate nomster;

#[macro_use]
extern crate structopt;

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
    nomster::parse(&contents);
}
