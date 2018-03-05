extern crate nomster;

#[macro_use]
extern crate nom;
#[macro_use]
extern crate structopt;

use nomster::{read_file, parser};

use std::char;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "FILE", help = "webster file (utf-8)", parse(from_os_str))]
    input: PathBuf,
}

named!(next_entry<&str, &str>, take_until!("<strong>"));
named!(strong_word<&str, &str>, delimited!(tag!("<strong>"), take_until!("</strong>"), tag!("</strong>")));

fn unicode_list(mut contents: &str) {
    let mut non_ascii_set = BTreeMap::new();
    while let Ok((remaining, _)) = next_entry(contents) {
        let (remaining, word) = strong_word(remaining).unwrap();
        contents = remaining;
        let word = parser::strip_stress(word);
        let mut u_iter = word.chars().map(|c| c as u32).peekable();
        while let Some(u) = u_iter.next() {
            if u >= 20 && u < 127 { continue; }
            let following = u_iter.peek().map_or(0, |&u| u);
            match non_ascii_set.entry(u) {
                Entry::Vacant(mut entry) => {
                    entry.insert(following);
                }
                Entry::Occupied(mut entry) => {
                    let prev_following = *entry.get();
                    if prev_following != 0 && prev_following != following {
                        entry.insert(0);
                    }
                }
            }
        }
    }
    let mut s = String::new();
    for (u, u_follow) in non_ascii_set.iter() {
        s.push(char::from_u32(*u).unwrap());
        if *u_follow > 0 {
            s.push(char::from_u32(*u_follow).unwrap());
        }
        println!("[{} {}]: {}", u, u_follow, s);
        s.truncate(0);
    }
}

fn main() {
    let opt = Opt::from_args();
    let contents = read_file(&opt.input).unwrap();
    unicode_list(&contents);
}
