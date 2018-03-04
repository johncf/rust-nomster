use nom::digit1;
use regex::Regex;

#[derive(Debug)]
pub struct RawEntry<'a> {
    pub word: &'a str,
    pub filepos: Option<u32>,
    pub body: &'a str,
    pub extras: &'a str,
}

pub enum FilePos<'a> {
    Def(u32),
    Ref(u32, &'a str),
    RefUnk(&'a str),
}

named!(entry_start<&str, &str>, take_until!("<idx:entry>"));

named!(pub filepos_num<&str, u32>, map!(digit1, |num| num.parse().unwrap()));

named!(pub filepos_ref_unk<&str, &str>,
       do_parse!(
           tag!("<a filepos=XXXXXXXXXX >") >>
           text: take_until_and_consume!("</a>") >>
           ( text )
      ));

named!(pub filepos_ref<&str, (u32, &str)>,
       do_parse!(
           tag!("<a href=\"#filepos") >>
           filepos: filepos_num >>
           tag!("\" >") >>
           text: take_until_and_consume!("</a>") >>
           ( (filepos, text) )
      ));

named!(pub filepos_def<&str, u32>,
       delimited!(
           tag!("<a "),
           delimited!(
               tag!("id=\"filepos"),
               filepos_num,
               tag!("\"")),
           tag!(" />")));

named!(pub a_tag<&str, FilePos>,
       alt!(map!(filepos_def, |fp| FilePos::Def(fp)) |
            map!(filepos_ref, |(fp, text)| FilePos::Ref(fp, text)) |
            map!(filepos_ref_unk, |text| FilePos::RefUnk(text))));

named!(pre_entry<&str, &str>,
       ws!(delimited!(
               tag!("<idx:entry>"),
               delimited!(
                   tag!("<idx:orth value=\""),
                   take_until!("\""),
                   take_until_and_consume!(">")),
               tag!("</idx:entry>"))));

named!(main_entry<&str, (&str, Option<u32>, &str)>,
       do_parse!(
           tag!("<p>") >>
               tag!("<big>") >>
                   filepos: opt!(filepos_def) >>
                   tag!("<b>") >>
                       word: take_until!("</b>") >>
                   tag!("</b>") >>
               tag!("</big>") >>
               body: take_until!("</p>") >>
           tag!("</p>") >>
           ( (word, filepos, body) )
      ));

pub fn next_entry(contents: &str) -> Option<(&str, Result<RawEntry, (&str, &str)>, &str)> {
    if let Ok((remaining, skipped)) = entry_start(contents) {
        let (remaining, idx_word) = pre_entry(remaining).expect("bad <idx:entry>");
        let pat = Regex::new(r"<idx:entry>|<mbp:pagebreak/>").unwrap();
        let m = pat.find(remaining).expect("entry did not end \"properly\"");
        let entry_str = &remaining[..m.start()];
        let remaining = &remaining[m.start()..];
        if let Ok((extras, (word, filepos, body))) = main_entry(entry_str) {
            //assert_eq!(idx_word, word); // fails for oe ligatures
            Some((skipped, Ok(RawEntry { word, filepos, body, extras }), remaining))
        } else {
            Some((skipped, Err((idx_word, entry_str)), remaining))
        }
    } else {
        None
    }
}

pub fn strip_stress(word: &str) -> String {
    word.replace(|c| c == '´' || c == '•', "")
}

#[cfg(test)]
mod test {
    use super::strip_stress;

    #[test]
    fn strip_stress_test() {
        assert_eq!(strip_stress("Law´giv•er"), "Lawgiver");
        assert_eq!(strip_stress("Zöll´ner’s lines"), "Zöllner’s lines");
        assert_eq!(strip_stress("Zee´man ef•fect´"), "Zeeman effect");
    }
}
