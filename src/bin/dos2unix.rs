extern crate nomster;

#[macro_use]
extern crate structopt;

use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "INFILE", help = "input file in dos format", parse(from_os_str))]
    input: PathBuf,
    #[structopt(name = "OUTFILE", help = "output path (overwrites INFILE if not given)",
                parse(from_os_str))]
    output: Option<PathBuf>,
}

fn write_unix(contents: &str, output: &Path) -> Result<(), std::io::Error> {
    use std::io::Write;
    let contents = contents.replace("\r\n", "\n");
    std::fs::File::create(output)?.write_all(contents.as_bytes())?;
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    let output = opt.output.as_ref().unwrap_or(&opt.input);
    let contents = nomster::read_dos(&opt.input).unwrap();
    write_unix(&contents, output).unwrap();
}
