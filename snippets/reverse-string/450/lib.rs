extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let grapheme = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    let mut rev: Vec<&str> = Vec::new();
    for c in grapheme{
        rev.push(c);
    }
    rev.reverse();
    let out = rev.join("");
    out
}
