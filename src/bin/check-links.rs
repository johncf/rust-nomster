extern crate nomster;

#[macro_use]
extern crate nom;

#[macro_use]
extern crate structopt;

use nomster::parser::{FilePos, next_entry, strip_stress, a_tag};
use std::collections::BTreeMap;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "FILE", help = "webster html file", parse(from_os_str))]
    input: PathBuf,
}

fn check_link_diversity(mut contents: &str) {
    let mut last_word_with_id = None;
    let mut diverse_links_count = 0;
    while let Some((_, entry, next)) = next_entry(contents) {
        if let Ok(entry) = entry {
            if entry.filepos.is_some() {
                if let Some(last_word) = last_word_with_id {
                    let word_sans_stress = strip_stress(entry.word);
                    if last_word == word_sans_stress {
                        println!("2 entries of the same word were both linked! {}", last_word);
                        diverse_links_count += 1;
                    }
                    last_word_with_id = Some(word_sans_stress);
                }
            }
        }
        contents = next;
    }
    println!(">>> There were {} diverse links", diverse_links_count);
}

named!(start<&str, &str>, take_until!("<idx:entry>"));
named!(next_link<&str, &str>, take_until!("<a "));
named!(bigb<&str, &str>, delimited!(tag!("<b>"), take_until!("</b>"), tag!("</b>")));

fn map_smart_insert(map: &mut BTreeMap<u32, (String, bool)>, key: u32, value: &str, def: bool) -> bool {
    let word = strip_stress(value).to_lowercase();
    let (is_diff, do_insert) = map.get(&key).map_or((false, true),
                                                    |&(ref w, d)| (w != &word, def || !d));
    if is_diff {
        println!("smart link? {:?} {:?}", map.get(&key).unwrap().0, word);
    }
    if do_insert {
        map.insert(key, (word, def));
    }
    return is_diff;
}

fn check_link_smartness(contents: &str) {
    let mut id_map = BTreeMap::new();
    let (mut contents, _) = start(contents).unwrap();
    let mut smart_count = 0;
    while let Ok((remaining, _)) = next_link(contents) {
        if let Ok((remaining, filepos)) = a_tag(remaining) {
            contents = remaining;
            match filepos {
                FilePos::Def(fp) => {
                    if let Ok((_, word)) = bigb(remaining) {
                        smart_count += map_smart_insert(&mut id_map, fp, word, true) as u32;
                    }
                }
                FilePos::Ref(fp, text) => {
                    smart_count += map_smart_insert(&mut id_map, fp, text, false) as u32;
                }
                FilePos::RefUnk(_) => (),
            }
        } else {
            eprintln!("Error near: {:?}", &remaining[..40]);
        }
    }
    println!(">>> There were {} possibly smart links", smart_count);
}

fn main() {
    let opt = Opt::from_args();
    let contents = nomster::read_file(&opt.input).unwrap();
    check_link_diversity(&contents);
    check_link_smartness(&contents);
}
