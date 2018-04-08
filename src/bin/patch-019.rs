extern crate nomster;

#[macro_use]
extern crate structopt;

use nomster::parser::{self, EntryTag, ParaTag, SimpleTag, TaggedEntry};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "INFILE", help = "webster html file", parse(from_os_str))]
    input: PathBuf,
    #[structopt(name = "OUTFILE", help = "patched output (default: overwrite)", parse(from_os_str))]
    output: Option<PathBuf>,
}

enum PatchState<'a> {
    WaitingDquotes(Vec<ParaTag<'a>>),
    WaitingAuthor(Vec<SimpleTag<'a>>),
}

fn process_ptag<'a>(entry_patched: &mut TaggedEntry<'a>, state: PatchState<'a>, ptag: ParaTag<'a>) -> PatchState<'a> {
    match state {
        PatchState::WaitingDquotes(mut ptags_patched) => {
            match ptag {
                ParaTag::Dquotes(mut qstags) => {
                    entry_patched.tags.push(EntryTag::Para(ptags_patched));
                    let last_idx = qstags.len()-1;
                    if let SimpleTag::Emph(author) = qstags[last_idx] {
                        if let SimpleTag::BreakTag = qstags[last_idx-1] {
                            qstags.pop(); qstags.pop();
                            entry_patched.tags.push(EntryTag::Blockquote(qstags, Some(author)));
                            PatchState::WaitingDquotes(Vec::new())
                        } else if let SimpleTag::Plain(prec) = qstags[last_idx-1] {
                            if prec.ends_with(" ") && author.ends_with(".") {
                                qstags.pop();
                                entry_patched.tags.push(EntryTag::Blockquote(qstags, Some(author)));
                                PatchState::WaitingDquotes(Vec::new())
                            } else {
                                PatchState::WaitingAuthor(qstags)
                            }
                        } else {
                            unreachable!("this can't be!");
                        }
                    } else {
                        PatchState::WaitingAuthor(qstags)
                    }
                }
                ptag => {
                    ptags_patched.push(ptag);
                    PatchState::WaitingDquotes(ptags_patched)
                }
            }
        }
        PatchState::WaitingAuthor(qstags) => {
            let mut author_opt = None;
            let mut ptag = ptag;
            if let ParaTag::Simple(ref mut stags) = ptag {
                match stags[0] {
                    SimpleTag::BreakTag | SimpleTag::Plain(" ") =>
                        if let SimpleTag::Emph(author) = stags[1] {
                            author_opt = Some(author);
                            println!("Author `{}`", author);
                            stags.drain(0..2);
                        },
                    _ => (),
                }
            }
            if author_opt.is_none() {
                println!("No author in {} around `{}`", entry_patched.word, qstags[0]);
            }
            entry_patched.tags.push(EntryTag::Blockquote(qstags, author_opt));
            process_ptag(entry_patched, PatchState::WaitingDquotes(Vec::new()), ptag)
        }
    }
}

fn write_html(out: &mut String, entry: TaggedEntry) {
    use std::fmt::Write;
    let mut entry_patched = TaggedEntry {
        tocid: entry.tocid,
        tags: Vec::new(),
        word: entry.word,
    };
    for etag in entry.tags {
        match etag {
            EntryTag::Para(ptags) => {
                let mut patch_state = PatchState::WaitingDquotes(Vec::new());
                for ptag in ptags {
                    patch_state = process_ptag(&mut entry_patched, patch_state, ptag);
                }
                match patch_state {
                    PatchState::WaitingDquotes(ptags_patched) => {
                        if ptags_patched.len() > 0 {
                            entry_patched.tags.push(EntryTag::Para(ptags_patched));
                        }
                    }
                    PatchState::WaitingAuthor(qstags) => {
                        println!("No author in {} around `{}`", entry_patched.word, qstags[0]);
                        entry_patched.tags.push(EntryTag::Blockquote(qstags, None));
                    }
                }
            }
            etag => entry_patched.tags.push(etag),
        }
    }
    write!(out, "{}", entry_patched).unwrap();
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
