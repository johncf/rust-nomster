extern crate nomster;

#[macro_use]
extern crate nom;

#[macro_use]
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "FILE", help = "gutenberg#673 in utf-8", parse(from_os_str))]
    input: PathBuf,
}

named!(next<&str, ()>, map!(take_until!("<hw>"), |_| ()));
named!(hw<&str, &str>, delimited!(tag!("<hw>"), take_until!("</hw>"), tag!("</hw>")));
named!(line<&str, &str>, take_until!("\n"));

fn list_hws(mut contents: &str) {
    while let Ok((remaining, _)) = next(contents) {
        let (remaining, mut line) = line(remaining).unwrap();
        contents = remaining;
        while let Ok((remaining, word)) = hw(line) {
            print!("{}", word);
            if let Ok((remaining, _)) = next(remaining) {
                print!("|");
                line = remaining;
            } else {
                break;
            }
        }
        println!();
    }
}

fn main() {
    let opt = Opt::from_args();
    let contents = nomster::read_file(&opt.input).unwrap();
    list_hws(&contents);
}
