use nom::hex_digit;

#[derive(Debug)]
pub struct RawEntry<'a> {
    pub word: &'a str,
    pub tocid: u32,
    pub body: &'a str,
    pub extras: &'a str,
}

#[derive(Debug)]
pub struct TaggedEntry<'a> {
    pub tocid: u32,
    pub tags: Vec<EntryTag<'a>>,
}

#[derive(Debug)]
pub enum EntryTag<'a> {
    Para(Vec<ParaTag<'a>>),
    Pre(&'a str),
}

#[derive(Debug)]
pub enum ParaTag<'a> {
    Strong(&'a str),
    Dquotes(Vec<SimpleTag<'a>>),
    Boxed(Vec<SimpleTag<'a>>),
    Simple(Vec<SimpleTag<'a>>),
}

#[derive(Debug)]
pub enum SimpleTag<'a> {
    Bold(&'a str),
    BoxedPlain(&'a str),
    BreakTag,
    Emph(&'a str),
    Plain(&'a str),
    SmallB(&'a str),
    Sub(&'a str),
    Sup(&'a str),
    WordRef(u32, &'a str),
}

fn toc_u32(toc: &str) -> u32 {
    u32::from_str_radix(toc, 16).unwrap()
}

named!(bold<&str, SimpleTag>,
       map!(delimited!(tag!("<b>"), is_not!("<>"), tag!("</b>")),
            |s| SimpleTag::Bold(s)));
named!(boxed_plain<&str, SimpleTag>,
       map!(delimited!(tag!("["), opt!(is_not!("<>[]")), tag!("]")),
            |s_o| SimpleTag::BoxedPlain(s_o.unwrap_or(""))));
named!(break_tag<&str, SimpleTag>,
       map!(tag!("<br>"), |_| SimpleTag::BreakTag));
named!(emph<&str, SimpleTag>,
       map!(delimited!(tag!("<i>"), take_until!("</i>"), tag!("</i>")),
            |s| SimpleTag::Emph(s)));
named!(small_b<&str, SimpleTag>,
       map!(delimited!(tag!("<small><b>"), is_not!("<>"), tag!("</b></small>")),
            |s| SimpleTag::SmallB(s)));
named!(sub<&str, SimpleTag>,
       map!(delimited!(tag!("<sub>"), is_not!("<>"), tag!("</sub>")),
            |s| SimpleTag::Sub(s)));
named!(sup<&str, SimpleTag>,
       map!(delimited!(tag!("<sup>"), is_not!("<>"), tag!("</sup>")),
            |s| SimpleTag::Sup(s)));
named!(plain<&str, SimpleTag>,
       map!(is_not!("<>[]“”"), |s| SimpleTag::Plain(s)));
named!(plain_nobox<&str, SimpleTag>,
       map!(is_not!("<>“”"), |s| SimpleTag::Plain(s)));
named!(word_ref<&str, SimpleTag>,
       map!(toc_link, |(id, text)| SimpleTag::WordRef(id, text)));

named!(quote_tags<&str, Vec<SimpleTag>>,
       many1!(alt!(plain_nobox | emph | break_tag)));

named!(boxed_tags<&str, Vec<SimpleTag>>,
       many1!(alt!(plain | emph | bold | word_ref | small_b | boxed_plain)));

named!(simple_tags<&str, Vec<SimpleTag>>,
       many1!(alt!(plain | emph | bold | word_ref | small_b | break_tag | boxed_plain | sub | sup)));

named!(strong<&str, ParaTag>,
       map!(delimited!(tag!("<strong>"), is_not!("<>"), tag!("</strong>")),
            |s| ParaTag::Strong(s)));
named!(dquotes<&str, ParaTag>,
       map!(delimited!(tag!("“"), quote_tags, tag!("”")),
            |v| ParaTag::Dquotes(v)));
named!(boxed<&str, ParaTag>,
       map!(delimited!(tag!("["), boxed_tags, tag!("]")),
            |v| ParaTag::Boxed(v)));
named!(simple<&str, ParaTag>,
       map!(simple_tags, |v| ParaTag::Simple(v)));

named!(pub parse_entry2<&str, TaggedEntry>,
       do_parse!(
           tocid: div_open >>
           tags: many1!(
               alt!(map!(delimited!(tag!("<p>"),
                                    many1!(alt!(simple | dquotes | boxed | strong)),
                                    tag!("</p>")),
                         |v| EntryTag::Para(v)) |
                    map!(delimited!(tag!("<pre>"),
                                    take_until!("</pre>"),
                                    tag!("</pre>")),
                         |s| EntryTag::Pre(s)))) >>
           ( TaggedEntry { tocid, tags } )));

named!(entry_start<&str, &str>, take_until!("<div id=\"MBP_"));

named!(pub toc_link<&str, (u32, &str)>,
       do_parse!(
           tag!("<a href=\"#MBP_TOC_") >>
           tocid: map!(hex_digit, toc_u32) >>
           tag!("\">") >>
           text: is_not!("<>") >>
           tag!("</a>") >>
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

pub fn next_entry2(contents: &str) -> Option<(&str, Result<TaggedEntry, &str>, &str)> {
    if let Ok((remaining, skipped)) = entry_start(contents) {
        let end_idx = remaining.find("</div>").expect("entry did not end properly") + 6;
        let entry_str = &remaining[..end_idx];
        let remaining = &remaining[end_idx..];
        if let Ok((_, entry_tags)) = parse_entry2(entry_str) {
            Some((skipped, Ok(entry_tags), remaining))
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
