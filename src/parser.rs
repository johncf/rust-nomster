use nom::hex_digit;
use std::fmt::{self, Display, Formatter};

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
    pub word: &'a str,
}

impl<'a> Display for TaggedEntry<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<div id=\"MBP_TOC_{id:X}\" data-ascii=\"{word}\">\n",
               id = self.tocid, word = self.word)?;
        for t in &self.tags {
            write!(f, "{}", t)?;
        }
        write!(f, "</div>\n")
    }
}

#[derive(Debug)]
pub enum EntryTag<'a> {
    Blockquote(Vec<SimpleTag<'a>>, Option<&'a str>),
    Para(Vec<ParaTag<'a>>),
    Pre(&'a str),
    LineBreak,
}

impl<'a> Display for EntryTag<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            EntryTag::Blockquote(ref tags, author_opt) => {
                write!(f, "<blockquote>\n<p>")?;
                for t in tags {
                    write!(f, "{}", t)?;
                }
                write!(f, "</p>\n")?;
                if let Some(author) = author_opt {
                    write!(f, "\u{2015}<i>{}</i>", author)?;
                }
                write!(f, "</blockquote>\n")?;
            }
            EntryTag::Para(ref tags) => {
                write!(f, "<p>")?;
                for t in tags {
                    write!(f, "{}", t)?;
                }
                write!(f, "</p>\n")?;
            }
            EntryTag::Pre(raw_html) => {
                write!(f, "<pre>{}</pre>", raw_html)?;
            }
            EntryTag::LineBreak => {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ParaTag<'a> {
    Strong(&'a str),
    Boxed(Vec<SimpleTag<'a>>),
    BoxedGrammar(Vec<SimpleTag<'a>>),
    Simple(Vec<SimpleTag<'a>>),
}

impl<'a> Display for ParaTag<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ParaTag::Strong(word) => {
                write!(f, "<strong>{}</strong>", word)?;
            }
            ParaTag::Boxed(ref tags) | ParaTag::BoxedGrammar(ref tags) => {
                write!(f, "[")?;
                for t in tags {
                    write!(f, "{}", t)?;
                }
                write!(f, "]")?;
            }
            ParaTag::Simple(ref tags) => {
                for t in tags {
                    write!(f, "{}", t)?;
                }
            }
        }
        Ok(())
    }
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

impl<'a> Display for SimpleTag<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            SimpleTag::Bold(text) => {
                write!(f, "<b>{}</b>", text)?;
            }
            SimpleTag::BoxedPlain(text) => {
                write!(f, "[{}]", text)?;
            }
            SimpleTag::BreakTag => {
                write!(f, "<br>\n")?;
            }
            SimpleTag::Emph(text) => {
                write!(f, "<i>{}</i>", text)?;
            }
            SimpleTag::Plain(text) => {
                write!(f, "{}", text)?;
            }
            SimpleTag::SmallB(text) => {
                write!(f, "<small><b>{}</b></small>", text)?;
            }
            SimpleTag::Sub(text) => {
                write!(f, "<sub>{}</sub>", text)?;
            }
            SimpleTag::Sup(text) => {
                write!(f, "<sup>{}</sup>", text)?;
            }
            SimpleTag::WordRef(id, text) => {
                write!(f, "<a href=\"#MBP_TOC_{:X}\">{}</a>", id, text)?;
            }
        }
        Ok(())
    }
}

fn toc_u32(toc: &str) -> u32 {
    u32::from_str_radix(toc, 16).unwrap()
}

fn is_gram_marker(text: &str) -> bool {
    let variants: &[&str] = &[
        //"1st pers.", "2d pers.", "3d pers.",
        //"a.",
        //"adv.",
        "compar.", "Compar.",
        "dat.",
        "imp.",
        //"indic.",
        "inf.",
        //"n.",
        "nom.",
        "obj.",
        "obs.", "Obs.",
        "p. p.",
        "p. pr.",
        //"pass.",
        "pl.",
        "poss.", "Poss.",
        //"prep.",
        "pres.",
        "pret.",
        //"pron.",
        "sing.", "Sing.",
        //"subj.",
        "superl.", "Superl.",
        //"v.",
        "v. i.",
        "v. t.",
        "vb. n.",
    ];
    for v in variants {
        if text.starts_with(v) {
            return true;
        }
    }
    false
}


named!(bold<&str, SimpleTag>,
       map!(delimited!(tag!("<b>"), is_not!("<>"), tag!("</b>")),
            |s| SimpleTag::Bold(s)));
named!(boxed_plain<&str, SimpleTag>,
       map!(delimited!(tag!("["), opt!(is_not!("<>[]")), tag!("]")),
            |s_o| SimpleTag::BoxedPlain(s_o.unwrap_or(""))));
named!(break_tag<&str, SimpleTag>,
       map!(tag!("<br>\n"), |_| SimpleTag::BreakTag));
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
       map!(is_not!("<>[]"), |s| SimpleTag::Plain(s)));
named!(plain_nobox<&str, SimpleTag>,
       map!(is_not!("<>"), |s| SimpleTag::Plain(s)));
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
named!(boxed<&str, ParaTag>,
       map!(delimited!(tag!("["), boxed_tags, tag!("]")),
            |v| match v[0] {
                SimpleTag::Emph(text) if is_gram_marker(text) => ParaTag::BoxedGrammar(v),
                _ => ParaTag::Boxed(v),
            }));
named!(simple<&str, ParaTag>,
       map!(simple_tags, |v| ParaTag::Simple(v)));

named!(parse_entry<&str, TaggedEntry>,
       do_parse!(
           divo: div_open >>
           tags: many1!(
               alt!(map!(delimited!(tag!("<p>"),
                                    many1!(alt!(simple | boxed | strong)),
                                    tag!("</p>\n")),
                         |v| EntryTag::Para(v)) |
                    map!(delimited!(tag!("<pre>"),
                                    take_until!("</pre>"),
                                    tag!("</pre>")),
                         |s| EntryTag::Pre(s)) |
                    map!(delimited!(tag!("<blockquote>\n"),
                                    tuple!(
                                        delimited!(tag!("<p>"), quote_tags, tag!("</p>\n")),
                                        opt!(delimited!(tag!("\u{2015}<i>"),
                                                        is_not!("<>"),
                                                        tag!("</i>")))),
                                    tag!("</blockquote>\n")),
                         |(v, a_o)| EntryTag::Blockquote(v, a_o)) |
                    map!(tag!("\n"), |_| EntryTag::LineBreak))) >>
           tag!("</div>\n") >>
           ( TaggedEntry { tocid: divo.0, tags: tags, word: divo.1 } )));

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

named!(div_open<&str, (u32, &str)>,
       do_parse!(
           tag!("<div id=\"MBP_TOC_") >>
           tocid: map!(hex_digit, toc_u32) >>
           tag!("\" data-ascii=\"") >>
           word: take_until_and_consume!("\">\n") >>
           ( tocid, word )
      ));

pub struct Parser<'a> {
    contents: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(contents: &'a str) -> Parser<'a> {
        Parser { contents }
    }

    pub fn remaining(&self) -> &'a str {
        self.contents
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = (&'a str, Result<TaggedEntry<'a>, &'a str>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok((remaining, skipped)) = entry_start(self.contents) {
            let end_idx = remaining.find("</div>\n").expect("entry did not end properly") + 7;
            let entry_str = &remaining[..end_idx];
            self.contents = &remaining[end_idx..];
            if let Ok((empty, entry_tags)) = parse_entry(entry_str) {
                assert!(empty.is_empty());
                Some((skipped, Ok(entry_tags)))
            } else {
                Some((skipped, Err(entry_str)))
            }
        } else {
            None
        }
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
