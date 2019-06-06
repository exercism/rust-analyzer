use std::str;

pub fn reverse(input: &str) -> String {
    // this doesn't work on wide chars as we are reversing the grapheme
    let b = input.as_bytes();
    let mut v = b.to_vec();
    v.reverse();
    str::from_utf8(&v).unwrap().to_string()
}
