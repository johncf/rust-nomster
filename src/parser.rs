use nom::hex_digit;
use regex::Regex;

#[derive(Debug)]
pub struct RawEntry<'a> {
    pub word: &'a str,
    pub tocid: u32,
    pub body: &'a str,
    pub extras: &'a str,
}

fn toc_u32(toc: &str) -> u32 {
    u32::from_str_radix(toc, 16).unwrap()
}

named!(entry_start<&str, &str>, take_until!("<p id=\"MBP_"));

named!(pub toc_link<&str, (u32, &str)>,
       do_parse!(
           tag!("<a href=\"#MBP_TOC_") >>
           tocid: map!(hex_digit, toc_u32) >>
           tag!("\">") >>
           text: take_until_and_consume!("</a>") >>
           ( (tocid, text) )
      ));

named!(pub entry_p<&str, u32>,
       delimited!(
           tag!("<p "),
           delimited!(
               tag!("id=\"MBP_TOC_"),
               map!(hex_digit, toc_u32),
               tag!("\"")),
           tag!(">")));

named!(main_entry<&str, (&str, u32, &str)>,
       do_parse!(
           tocid: entry_p >>
               tag!("<big><b>") >>
                   word: take_until!("</b>") >>
               tag!("</b></big>") >>
               body: take_until!("</p>") >>
           tag!("</p>") >>
           ( (word, tocid, body) )
      ));

pub fn next_entry(contents: &str) -> Option<(&str, Result<RawEntry, &str>, &str)> {
    if let Ok((remaining, skipped)) = entry_start(contents) {
        let pat = Regex::new(r#"<p id="MBP_TOC_|<p class="breakhere|</body>"#).unwrap();
        let m = pat.find(&remaining[1..]).expect("entry did not end \"properly\"");
        let entry_str = &remaining[..m.start()+1];
        let remaining = &remaining[m.start()+1..];
        if let Ok((extras, (word, tocid, body))) = main_entry(entry_str) {
            Some((skipped, Ok(RawEntry { word, tocid, body, extras }), remaining))
        } else {
            Some((skipped, Err(entry_str), remaining))
        }
    } else {
        None
    }
}

fn is_stress(c: char) -> bool {
    c == '´' || c == '•'
}

pub fn strip_stress(word: &str) -> String {
    word.replace(is_stress, "")
}

/// also strips off stress/accent markers
pub fn to_ascii(word: &str) -> String {
    let s: String = word.chars()
                        .filter(|&c| !is_stress(c))
                        .map(|c| match c as u32 {
                            199 => 'C', 224 => 'a', 225 => 'a', 226 => 'a',
                            227 => 'a', 228 => 'a', 231 => 'a', 232 => 'e',
                            234 => 'e', 235 => 'e', 238 => 'i', 239 => 'i',
                            241 => 'n', 243 => 'o', 244 => 'o', 246 => 'o',
                            249 => 'u', 251 => 'u', 252 => 'u', 8217=> '\'',
                            _ => c,
                        }).collect();
    s.replace('\u{0152}', "OE").replace('\u{0153}', "oe")
}

/// applies to_ascii, to_lowercase, replace spaces with underscores, remove quotes
pub fn to_id(word: &str) -> String {
    to_ascii(word).to_lowercase().replace(' ', "_").replace('\'', "")
}

#[cfg(test)]
mod test {
    use super::{strip_stress, to_ascii, to_id};

    #[test]
    fn strip_stress_test() {
        assert_eq!(strip_stress("A•mœ´ba"), "Amœba");
        assert_eq!(strip_stress("Law´giv•er"), "Lawgiver");
        assert_eq!(strip_stress("Zöll´ner’s Lines"), "Zöllner’s Lines");
        assert_eq!(strip_stress("Zee´man ef•fect´"), "Zeeman effect");
    }

    #[test]
    fn to_ascii_test() {
        assert_eq!(to_ascii("A•mœ´ba"), "Amoeba");
        assert_eq!(to_ascii("Law´giv•er"), "Lawgiver");
        assert_eq!(to_ascii("Zöll´ner’s Lines"), "Zollner's Lines");
        assert_eq!(to_ascii("Zee´man-ef•fect´"), "Zeeman-effect");
    }

    #[test]
    fn to_id_test() {
        assert_eq!(to_id("A•mœ´ba"), "amoeba");
        assert_eq!(to_id("Law´giv•er"), "lawgiver");
        assert_eq!(to_id("Zöll´ner’s Lines"), "zollners_lines");
        assert_eq!(to_id("Zee´man-ef•fect´"), "zeeman-effect");
    }
}
