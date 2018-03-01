extern crate nomster;

#[macro_use]
extern crate nom;
#[macro_use]
extern crate structopt;

use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "FILE", help = "webster file (utf-8)", parse(from_os_str))]
    input: PathBuf,
    #[structopt(name = "OUTFILE", help = "patched output (overwrites FILE if not given)",
                parse(from_os_str))]
    output: Option<PathBuf>,
}

named!(till_hwhw<&str, &str>, take_until!("<hw><hw>"));
named!(till_hw<&str, &str>, take_until!("<hw>"));
named!(till_curly<&str, &str>, is_not!("}"));
named!(till_linebreak<&str, &str>, is_not!("\n"));
named!(hw_proper<&str, (&str, &str, &str)>, tuple!(tag!("<hw>"), is_not!("<>"), tag!("</hw>")));

fn patch(mut contents: &str, output: &Path) -> Result<(), std::io::Error> {
    use std::io::Write;
    let mut patched = String::with_capacity(contents.len());
    loop {
        match till_hwhw(contents) {
            Ok((remaining, consumed)) => {
                patched.push_str(consumed);
                let (remaining, mut line) =
                    till_linebreak(&remaining[4..]).expect("expected linebreak not found");
                contents = remaining;
                loop {
                    match hw_proper(line) {
                        Ok((remaining, (open, inside, close))) => {
                            patched.push_str(open);
                            patched.push_str(inside);
                            patched.push_str(close);
                            line = remaining;
                        }
                        Err(_) => { // unclosed <hw>
                            patched.push_str(&line[4..]);
                            break;
                        }
                    }
                    match till_hw(line) {
                        Ok((remaining, consumed)) => {
                            patched.push_str(consumed);
                            line = remaining;
                        }
                        Err(_) => {
                            // no more <hw>; there must be a '}' then.
                            let (remaining, consumed) =
                                till_curly(line).expect("expected '}' not found");
                            patched.push_str(consumed);
                            patched.push_str(&remaining[1..]);
                            break;
                        }
                    }
                }
            }
            Err(_) => {
                patched.push_str(contents);
                break;
            }
        }
    }
    std::fs::File::create(output)?.write_all(patched.as_bytes())?;
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    let output = opt.output.as_ref().unwrap_or(&opt.input);
    let contents = nomster::read_unix(&opt.input).unwrap();
    patch(&contents, output).unwrap();
}
