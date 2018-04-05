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

fn write_html(out: &mut String, mut entry: parser::TaggedEntry) {
    use std::fmt::Write;
    for etag in &mut entry.tags {
        if let parser::EntryTag::Para(ref mut ptags) = *etag {
            for ptag in ptags {
                if let parser::ParaTag::BoxedGrammar(ref mut btags) = *ptag {
                    let mut keep_ref = false;
                    for stag in btags {
                        use nomster::parser::SimpleTag;
                        match *stag {
                            SimpleTag::Plain(text) => {
                                if keep_ref {
                                    keep_ref = text.trim() == ",";
                                } else {
                                    keep_ref = text.ends_with("See ") || text.ends_with("see ") ||
                                               text.ends_with("of ");
                                }
                            }
                            SimpleTag::WordRef(_, text) => {
                                if !keep_ref {
                                    *stag = SimpleTag::SmallB(text);
                                } else {
                                    println!("kept ref to {} in {}", text, entry.word);
                                }
                            }
                            _ => keep_ref = false,
                        }
                    }
                }
            }
        }
    }
    write!(out, "{}", entry).unwrap();
}

fn patch(mut contents: &str, output: &Path) -> Result<(), std::io::Error> {
    use std::io::Write;
    let mut patched = String::with_capacity(contents.len());
    while let Some((skipped, entry, next)) = parser::next_entry2(contents) {
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
