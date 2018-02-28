use nom::{alpha, digit, line_ending, multispace0};

named!(start<&str, &str>, take_until!("<hr>"));

named!(page_sep<&str, u32>,
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
        ( pgno_parse(page, page_dup) )
    ));

named!(alphabet<&str, &str>,
    do_parse!(
        multispace0 >>
        a: delimited!(
            tag!("<centered>"),
            delimited!(
                tag!("<point26>"),
                terminated!(alpha, char!('.')),
                tag!("</point26>")),
            tag!("</centered>")) >>
        multispace0 >>
        ( a )
    ));

fn pgno_parse(page: &str, page_dup: &str) -> u32 {
    let page = page.parse();
    assert_eq!(page, page_dup.parse());
    page.unwrap()
}

pub fn parse(contents: &str) {
    let (next, consumed) = start(contents).unwrap();
    println!("{} bytes header, {} bytes remaining", consumed.len(), next.len());
    let (next, page) = page_sep(next).unwrap();
    println!("page {}, {} bytes remaining", page, next.len());
    let (next, alphabet) = alphabet(next).unwrap();
    println!("alphabet {}, {} bytes remaining", alphabet, next.len());
}
