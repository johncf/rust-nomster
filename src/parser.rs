use nom::digit1;

named!(start<&str, &str>, take_until!("<idx:entry>"));

named!(pub filepos_def<&str, u32>,
       delimited!(
           tag!("<a "),
           delimited!(
               tag!("id=\"filepos"),
               map!(digit1, |num| num.parse().unwrap()),
               tag!("\"")),
           tag!(" />")));

pub fn parse(contents: &str) {
    let (contents, consumed) = start(contents).unwrap();
    println!("consumed {} bytes, remaining {} bytes", consumed.len(), contents.len());
}
