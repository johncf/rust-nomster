named!(start<&str, &str>, take_until!("<idx:entry>"));

pub fn parse(contents: &str) {
    let (contents, consumed) = start(contents).unwrap();
    println!("consumed {} bytes, remaining {} bytes", consumed.len(), contents.len());
}
