extern crate encoding;
#[macro_use]
extern crate structopt;

use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};
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

pub fn read_dos<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let mut contents = Vec::with_capacity(2 << 20);
    File::open(path)?.read_to_end(&mut contents)?;
    let (dec_res, enc_ref) = encoding::decode(&contents,
                                              encoding::DecoderTrap::Strict,
                                              encoding::all::WINDOWS_1252);
    dec_res.map_err(|e| {
        Error::new(ErrorKind::InvalidData,
                   format!("{} decoding error: {}", enc_ref.name(), e))
    })
}

fn write_unix(contents: &str, output: &Path) -> Result<(), Error> {
    let contents = contents.replace("\r\n", "\n");
    File::create(output)?.write_all(contents.as_bytes())?;
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    let output = opt.output.as_ref().unwrap_or(&opt.input);
    let contents = read_dos(&opt.input).unwrap();
    write_unix(&contents, output).unwrap();
}
