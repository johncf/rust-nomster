extern crate nomster;

#[macro_use]
extern crate nom;

#[macro_use]
extern crate structopt;

use nomster::parser::filepos_def;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "FILE", help = "webster html file", parse(from_os_str))]
    input: PathBuf,
}

named!(next<&str, ()>, map!(take_until!("<a id=\"filepos"), |_| ()));

fn max_filepos(mut contents: &str) -> u32 {
    let mut max_filepos = 0;
    while let Ok((remaining, _)) = next(contents) {
        let (remaining, filepos) = filepos_def(remaining).unwrap();
        contents = remaining;
        if filepos > max_filepos {
            max_filepos = filepos;
        }
    }
    max_filepos
}

fn main() {
    let opt = Opt::from_args();
    let contents = nomster::read_file(&opt.input).unwrap();
    println!("biggest filepos: {}", max_filepos(&contents));
}
