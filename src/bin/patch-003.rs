extern crate nomster;

#[macro_use]
extern crate nom;

#[macro_use]
extern crate structopt;

use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "INFILE", help = "webster html file", parse(from_os_str))]
    input: PathBuf,
    #[structopt(name = "OUTFILE", help = "patched output (default: overwrite)", parse(from_os_str))]
    output: Option<PathBuf>,
}

struct SmallWords<'a> {
    word: &'a str,
    alts: &'a str,
}

named!(small_words<&str, SmallWords>,
       do_parse!(
           word: delimited!(
               tag!("<small style=\"color: blue\"><b style=\"color: blue\">"),
               take_until!("</b>"),
               tag!("</b></small>")) >>
           alts: take_until!("}") >>
           ( SmallWords { word, alts } )
      ));

named!(small_words_in_curly<&str, SmallWords>,
       delimited!(
           ws!(tag!("{")),
           small_words,
           tag!("}")));

named!(next<&str, &str>, take_until!("{"));

named!(skip_line<&str, &str>, take_until!("\n"));

fn patch(mut contents: &str, output: &Path) -> Result<(), std::io::Error> {
    use std::io::Write;
    let mut patched = String::with_capacity(contents.len());
    while let Ok((remaining, consumed)) = next(contents) {
        patched.push_str(consumed);
        if let Ok((remaining, words)) = small_words_in_curly(remaining) {
            contents = remaining;
            patched.push_str(&format!("</p><idx:entry>\n<idx:orth value=\"{word}\" data-mark=\"new\">\n</idx:entry>\n<p><big><b>{word}</b></big>{alts}", word = words.word, alts = words.alts));
        } else if let Ok((remaining, line)) = skip_line(remaining) {
            contents = remaining;
            eprintln!(">>> badly formatted curly: {:?}", &line);
            patched.push_str(line);
        } else {
            contents = remaining;
            eprintln!(">>> badly formatted curly in the final line: {:?}", remaining);
            break;
        }
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
