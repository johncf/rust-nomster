extern crate parster;

#[macro_use]
extern crate structopt;

use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "main")]
struct Opt {
    #[structopt(name = "FILE", help = "webster file (utf-8)", parse(from_os_str))]
    input: PathBuf,
}

pub fn read_contents(path: &Path) -> Result<String, std::io::Error> {
    use std::io::Read;
    let mut contents = String::with_capacity(2 << 20);
    std::fs::File::open(path)?.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let opt = Opt::from_args();
    let contents = read_contents(&opt.input).unwrap();
    parster::parse(&contents);
}
