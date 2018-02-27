extern crate encoding;
#[macro_use]
extern crate nom;

use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::Path;

use encoding::{DecoderTrap, decode};
use encoding::all::WINDOWS_1252 as DEFAULT_ENCODING;

mod parser;

fn get_contents<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let mut contents = Vec::with_capacity(2 << 20);
    File::open(path)?.read_to_end(&mut contents)?;
    let (dec_res, enc_ref) = decode(&contents, DecoderTrap::Strict, DEFAULT_ENCODING);
    dec_res.map_err(|e| Error::new(ErrorKind::InvalidData,
                                   format!("{} decoding error: {}", enc_ref.name(), e)))
}

fn main() {
    let contents = get_contents("webster/673.txt").unwrap();
    let (next, consumed) = parser::start(&contents).unwrap();
    println!("{} bytes consumed, {} bytes remaining", consumed.len(), next.len());
}
