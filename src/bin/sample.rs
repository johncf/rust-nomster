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
    let mut contents = &*contents;
    while let Some((skipped, entry, next)) = nomster::parser::next_entry(contents) {
        match entry {
            Ok(entry) => {
                println!("{} (skipped {} bytes)", entry.word, skipped.len());
                println!("  tocid {:?}", entry.tocid);
                println!("  body {} bytes", entry.body.len());
                println!("  extras {} bytes", entry.extras.len());
            }
            Err(entry_str) => {
                println!("--parsing failed-- (skipped {} bytes)", skipped.len());
                println!("  entry size {} bytes", entry_str.len());
            }
        }
        contents = next;
    }
}
