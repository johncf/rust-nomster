#[cfg(feature="dosformat")]
extern crate encoding;
#[macro_use]
extern crate nom;

use std::io::{Error, Read};
use std::fs::File;
use std::path::Path;

mod parser;

pub use parser::parse;

#[cfg(feature="dosformat")]
pub fn read_dos<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let mut contents = Vec::with_capacity(2 << 20);
    File::open(path)?.read_to_end(&mut contents)?;
    let (dec_res, enc_ref) = encoding::decode(&contents,
                                              encoding::DecoderTrap::Strict,
                                              encoding::all::WINDOWS_1252);
    dec_res.map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::InvalidData,
                            format!("{} decoding error: {}", enc_ref.name(), e))
    })
}

pub fn read_unix<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let mut contents = String::with_capacity(2 << 20);
    File::open(path)?.read_to_string(&mut contents)?;
    Ok(contents)
}
