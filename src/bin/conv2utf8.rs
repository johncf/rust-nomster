extern crate parster;

#[macro_use]
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "main")]
struct Opt {
    #[structopt(name = "INFILE", help = "webster raw file (dos format)", parse(from_os_str))]
    input: PathBuf,
    #[structopt(name = "OUTFILE", help = "output file path", parse(from_os_str))]
    output: PathBuf,
}

pub fn write_unix<P: AsRef<std::path::Path>>(contents: &str, output: P) -> Result<(), std::io::Error> {
    use std::io::Write;
    let contents = contents.replace("\r\n", "\n");
    std::fs::File::create(output)?.write_all(contents.as_bytes())?;
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    let contents = parster::read_dos(opt.input).unwrap();
    write_unix(&contents, opt.output).unwrap();
}
