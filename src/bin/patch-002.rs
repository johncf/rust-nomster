extern crate nomster;

#[macro_use]
extern crate nom;

#[macro_use]
extern crate structopt;

use nom::digit1;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "INFILE", help = "webster html file", parse(from_os_str))]
    input: PathBuf,
    #[structopt(name = "OUTFILE", help = "patched output (default: overwrite)", parse(from_os_str))]
    output: Option<PathBuf>,
}

named!(filepos_def<&str, u32>,
       delimited!(
           tag!("<a "),
           delimited!(
               tag!("id=\"filepos"),
               map!(digit1, |num| num.parse().unwrap()),
               tag!("\"")),
           tag!(" />")));

named!(filepos_deflist<&str, Vec<u32>>,
       fold_many0!(filepos_def, Vec::new(), |mut acc: Vec<_>, id: u32| {
           if !acc.contains(&id) {
               acc.push(id);
           }
           acc
       }));

named!(next<&str, &str>, take_until!("<a id=\"filepos"));

fn patch(mut contents: &str, output: &Path) -> Result<(), std::io::Error> {
    use std::io::Write;
    let mut patched = String::with_capacity(contents.len());
    while let Ok((remaining, consumed)) = next(contents) {
        patched.push_str(consumed);
        let (remaining, ids) = filepos_deflist(remaining).unwrap();
        contents = remaining;
        if ids.len() > 1 {
            println!("more than one ids found: {:?}", ids);
        }
        for id in ids {
            patched.push_str(&format!("<a id=\"filepos{}\" />", id));
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
