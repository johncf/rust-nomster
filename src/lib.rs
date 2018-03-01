#[cfg(feature="dosformat")]
extern crate encoding;
#[macro_use]
extern crate nom;

mod parser;

pub use parser::parse;

#[cfg(feature="dosformat")]
pub fn read_dos<P: AsRef<std::path::Path>>(path: P) -> Result<String, std::io::Error> {
    use std::io::Read;
    let mut contents = Vec::with_capacity(2 << 20);
    std::fs::File::open(path)?.read_to_end(&mut contents)?;
    let (dec_res, enc_ref) = encoding::decode(&contents,
                                              encoding::DecoderTrap::Strict,
                                              encoding::all::WINDOWS_1252);
    dec_res.map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::InvalidData,
                            format!("{} decoding error: {}", enc_ref.name(), e))
    })
}
