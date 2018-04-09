extern crate nomster;

#[macro_use]
extern crate structopt;

use nomster::Parser;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "INFILE", help = "webster html file", parse(from_os_str))]
    input: PathBuf,
    #[structopt(name = "OUTFILE", help = "output file (default: overwrite)", parse(from_os_str))]
    output: Option<PathBuf>,
}

fn patch(contents: &str, output: &Path) -> Result<(), std::io::Error> {
    use std::io::Write;
    let mut patched = String::with_capacity(contents.len());
    let mut entry_iter = Parser::new(contents);
    while let Some((skipped, entry)) = entry_iter.next() {
        use std::fmt::Write;
        patched.push_str(skipped);
        let entry = entry.unwrap();
        write!(patched, "{}", entry).unwrap();
    }
    patched.push_str(entry_iter.remaining());
    std::fs::File::create(output)?.write_all(patched.as_bytes())?;
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    let output = opt.output.as_ref().unwrap_or(&opt.input);
    let contents = nomster::read_file(&opt.input).unwrap();
    patch(&contents, &output).unwrap();
}
