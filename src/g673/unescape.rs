extern crate nomster;

#[macro_use]
extern crate structopt;

#[macro_use]
extern crate lazy_static;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "INFILE", help = "input file (utf-8)", parse(from_os_str))]
    input: PathBuf,
    #[structopt(name = "OUTFILE", help = "output path", parse(from_os_str))]
    output: PathBuf,
}

lazy_static! {
    static ref SYMMAP: Vec<(&'static str, &'static str, &'static str)> = vec![
        ("\\'3c", "\u{003C}", "<"),
        ("\\'3e", "\u{003E}", ">"),
        ("\\'80", "\u{00C7}", "C"),
        ("\\'81", "\u{00FC}", "ue"),
        ("\\'82", "\u{00E9}", "e"),
        ("\\'83", "\u{00E2}", "a"),
        ("\\'84", "\u{00E4}", "ae"),
        ("\\'85", "\u{00E0}", "a"),
        ("\\'86", "\u{00E5}", "a"),
        ("\\'87", "\u{00E7}", "c"),
        ("\\'88", "\u{00EA}", "e"),
        ("\\'89", "\u{00EB}", "e"),
        ("\\'8a", "\u{00E8}", "e"),
        ("\\'8b", "\u{00EF}", "i"),
        ("\\'8c", "\u{00EE}", "i"),
        ("\\'8d", "\u{00EC}", "i"),
        ("\\'8e", "\u{00C4}", "A"),
        ("\\'90", "\u{00C9}", "e"),
        ("\\'91", "\u{00E6}", "ae"),
        ("\\'92", "\u{00C6}", "AE"),
        ("\\'93", "\u{00F4}", "o"),
        ("\\'94", "\u{00F6}", "oe"),
        ("\\'95", "\u{00F2}", "o"),
        ("\\'96", "\u{00FB}", "u"),
        ("\\'97", "\u{00F9}", "u"),
        ("\\'98", "\u{00FF}", "y"),
        ("\\'99", "\u{00D6}", "O"),
        ("\\'9a", "\u{00DC}", "U"),
        ("\\'9c", "\u{00A3}", "*"),
        ("\\'a0", "\u{00E1}", "a"),
        ("\\'a1", "\u{00ED}", "i"),
        ("\\'a2", "\u{00F3}", "o"),
        ("\\'a3", "\u{00FA}", "u"),
        ("\\'a4", "\u{00F1}", "ny"),
        ("\\'a5", "\u{00D1}", "NY"),
        ("\\'a9", "\u{2033}", "."),
        ("\\'b0", "\u{FFFD}", "(?)"),
        ("\\'b7", "\u{2032}", "'"),
        ("\\'b8", "\u{201D}", "\""),
        ("\\'bc", "\u{00A7}", "*"),
        ("\\'bd", "\u{201C}", "\""),
        ("\\'be", "\u{0101}", "a"),
        ("\\'bf", "\u{2018}", "`"),
        ("\\'c1", "\u{266F}", "#"),
        ("\\'c2", "\u{266D}", "*"),
        ("\\'c6", "\u{012B}", "i"),
        ("\\'c7", "\u{0113}", "e"),
        ("\\'cb", "\u{0115}", "e"),
        ("\\'cc", "\u{012D}", "i"),
        ("\\'ce", "\u{014F}", "o"),
        ("\\'cf", "\u{2013}", "-"),
        ("\\'d0", "\u{2014}", "--"),
        ("\\'d1", "\u{0152}", "OE"),
        ("\\'d2", "\u{0153}", "oe"),
        ("\\'d3", "\u{014D}", "o"),
        ("\\'d4", "\u{016B}", "u"),
        ("\\'d5", "\u{01D2}", "o"),
        ("\\'d6", "\u{01E3}", "ae"),
        ("\\'d8", "\u{2225}", "||"),
        ("\\'dc", "\u{016D}", "u"),
        ("\\'dd", "\u{0103}", "a"),
        ("\\'de", "\u{02d8}", "~"),
        ("\\'df", "\u{0233}", "y"),
        ("\\'ed", "\u{00FE}", "th"),
        ("\\'ee", "\u{00E3}", "a"),
        ("\\'f5", "\u{2014}", "--"),
        ("\\'f6", "\u{00F7}", "/"),
        ("\\'f7", "\u{2248}", "~="),
        ("\\'f8", "\u{00B0}", "*"),
        ("\\'f9", "\u{2022}", "*"),
        ("\\'fb", "\u{221A}", "*"),
    ];
}

fn to_unicode(contents: &str) -> String {
    let mut contents = contents.replace(SYMMAP[0].0, SYMMAP[0].1);
    for &(pat, rep, _) in &SYMMAP[1..] {
        contents = contents.replace(pat, rep);
    }
    contents
}

fn write_decoded(contents: &str, output: &PathBuf) {
    use std::io::Write;
    let contents = to_unicode(contents);
    std::fs::File::create(output).unwrap().write_all(contents.as_bytes()).unwrap();
}

fn main() {
    let opt = Opt::from_args();
    let contents = nomster::read_file(&opt.input).unwrap();
    write_decoded(&contents, &opt.output);
}
