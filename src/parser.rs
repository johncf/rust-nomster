use nom::hex_digit;

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

named!(entry_start<&str, &str>, take_until!("<div id=\"MBP_"));

named!(pub toc_link<&str, (u32, &str)>,
       do_parse!(
           tag!("<a href=\"#MBP_TOC_") >>
           tocid: map!(hex_digit, toc_u32) >>
           tag!("\">") >>
           text: take_until_and_consume!("</a>") >>
           ( (tocid, text) )
      ));

named!(div_open<&str, u32>,
       do_parse!(
           tag!("<div id=\"MBP_TOC_") >>
           tocid: map!(hex_digit, toc_u32) >>
           tag!("\" data-ascii=") >>
           take_until_and_consume!(">\n") >>
           ( tocid )
      ));

named!(pub parse_entry<&str, RawEntry>,
       do_parse!(
           tocid: div_open >>
           tag!("<p>") >>
               tag!("<strong>") >>
                   word: take_until!("</strong>") >>
               tag!("</strong>") >>
               body: take_until!("</p>") >>
           tag!("</p>") >>
           extras: take_until!("</div>") >>
           tag!("</div>") >>
           ( RawEntry { word, tocid, body, extras } )
      ));

pub fn next_entry(contents: &str) -> Option<(&str, Result<RawEntry, &str>, &str)> {
    if let Ok((remaining, skipped)) = entry_start(contents) {
        let end_idx = remaining.find("</div>").expect("entry did not end properly") + 6;
        let entry_str = &remaining[..end_idx];
        let remaining = &remaining[end_idx..];
        if let Ok((_, entry)) = parse_entry(entry_str) {
            Some((skipped, Ok(entry), remaining))
        } else {
            Some((skipped, Err(entry_str), remaining))
        }
    } else {
        None
    }
}

pub fn strip_stress(word: &str) -> String {
    word.replace(|c| c == '´' || c == '•', "")
}

/// lexicographic translation to ascii
pub fn word_to_ascii(word: &str) -> String {
    let mut word = strip_stress(word);
    word = word.chars().map(|c| match c as u32 {
                                    199 => 'C', 224 => 'a', 225 => 'a', 226 => 'a',
                                    227 => 'a', 228 => 'a', 231 => 'a', 232 => 'e',
                                    234 => 'e', 235 => 'e', 238 => 'i', 239 => 'i',
                                    241 => 'n', 243 => 'o', 244 => 'o', 246 => 'o',
                                    249 => 'u', 251 => 'u', 252 => 'u',
                                    7497 => 'e', 7511 => 't', 8217 => '\'',
                                    _ => c,
                                }).collect();
    word = word.replace('\u{0152}', "OE").replace('\u{0153}', "oe");
    assert!(word.is_ascii());
    word
}

#[cfg(test)]
mod test {
    use super::{strip_stress, word_to_ascii};

    #[test]
    fn strip_stress_test() {
        assert_eq!(strip_stress("A•mœ´ba"), "Amœba");
        assert_eq!(strip_stress("Law´giv•er"), "Lawgiver");
        assert_eq!(strip_stress("Zöll´ner’s Lines"), "Zöllner’s Lines");
        assert_eq!(strip_stress("Zee´man ef•fect´"), "Zeeman effect");
    }

    #[test]
    fn word_to_ascii_test() {
        assert_eq!(word_to_ascii("A•mœ´ba"), "Amoeba");
        assert_eq!(word_to_ascii("Law´giv•er"), "Lawgiver");
        assert_eq!(word_to_ascii("Zöll´ner’s Lines"), "Zollner's Lines");
        assert_eq!(word_to_ascii("Zee´man-ef•fect´"), "Zeeman-effect");
    }
}
