extern crate nomster;

#[macro_use]
extern crate structopt;

use nomster::parser;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "INFILE", help = "webster html file", parse(from_os_str))]
    input: PathBuf,
    #[structopt(name = "OUTFILE", help = "patched output (default: overwrite)", parse(from_os_str))]
    output: Option<PathBuf>,
}

fn write_html(out: &mut String, entry: parser::RawEntry) {
    use std::fmt::Write;
    let ascii_word = parser::word_to_ascii(entry.word);
    assert!(!ascii_word.contains('"'));
    write!(out, "<div id=\"MBP_TOC_{id:X}\" data-ascii=\"{ascii}\">\n<p><strong>{word}</strong>{body}</p>{extras}</div>\n",
           id = entry.tocid, ascii = ascii_word, word = entry.word, body = entry.body, extras = entry.extras).unwrap();
}

fn patch(mut contents: &str, output: &Path) -> Result<(), std::io::Error> {
    use std::io::Write;
    let mut patched = String::with_capacity(contents.len());
    while let Some((skipped, entry, next)) = parser::next_entry(contents) {
        contents = next;
        patched.push_str(skipped);
        let entry = entry.unwrap();
        write_html(&mut patched, entry);
    }
    patched.push_str(contents);
    std::fs::File::create(output)?.write_all(patched.as_bytes())?;
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    let output = opt.output.as_ref().unwrap_or(&opt.input);
    let contents = nomster::read_file(&opt.input).unwrap();
    patch(&contents, &output).unwrap();
}
