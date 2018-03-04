use nom::digit1;
use regex::Regex;

#[derive(Debug)]
pub struct RawEntry<'a> {
    pub word: &'a str,
    pub filepos: Option<u32>,
    pub body: &'a str,
    pub extras: &'a str,
}

named!(entry_start<&str, &str>, take_until!("<idx:entry>"));

named!(pub filepos_def<&str, u32>,
       delimited!(
           tag!("<a "),
           delimited!(
               tag!("id=\"filepos"),
               map!(digit1, |num| num.parse().unwrap()),
               tag!("\"")),
           tag!(" />")));

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
