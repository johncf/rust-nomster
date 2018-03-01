use nom::{digit, line_ending, multispace0};

named!(start<&str, &str>, take_until!("<hr>"));

named!(page_proper_mark<&str, u16>,
    do_parse!(
        tag!("<hr>") >>
        line_ending >>
        tag!("<page=\"") >>
        page: digit >>
        tag!("\">") >>
        line_ending >>
        tag!("Page ") >>
        page_dup: digit >>
        tag!("<p>") >>
        line_ending >>
        ( { assert_eq!(page, page_dup); pgno_parse(page) } )
    ));

named!(page_comment_mark<&str, u16>,
    do_parse!(
        multispace0 >>
        page: delimited!(
            tag!("<--"),
            ws!(delimited!(tag!("p."), digit, take_until!("-->"))),
            tag!("-->")) >>
        ( pgno_parse(page) )
    ));

named!(page_mark<&str, u16>, alt!(page_proper_mark | page_comment_mark));

named!(plaintext<&str, &str>, is_not!("<>"));

named!(xpage<&str, u16>,
    delimited!(
        tag!("<Xpage="),
        map!(digit, pgno_parse),
        tag!(">")
    ));

named!(mainword<&str, (&str, u16)>,
    do_parse!(
        multispace0 >>
        word: delimited!(
            tag!("<h1>"),
            plaintext,
            tag!("</h1>")) >>
        multispace0 >>
        page: xpage >>
        ( word, page )
    ));

fn pgno_parse(page: &str) -> u16 {
    page.parse().unwrap()
}

pub fn parse(contents: &str) {
    let (next, consumed) = start(contents).unwrap();
    println!("{} bytes header, {} bytes remaining", consumed.len(), next.len());
    let (next, page) = page_mark(next).unwrap();
    println!("page {}, {} bytes remaining", page, next.len());
    if let Ok((next, wordpage)) = mainword(next) {
        println!("word&page {:?}, {} bytes remaining", wordpage, next.len());
    } else {
        println!("parsing failed!");
    }
}
