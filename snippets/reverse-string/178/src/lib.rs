extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let v: Vec<&str> = UnicodeSegmentation::graphemes(input, true).rev().collect();
    let mut r = String::new();
    for x in v {
        r.push_str(x);
    }
    r
}
