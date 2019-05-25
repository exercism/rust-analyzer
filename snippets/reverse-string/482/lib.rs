extern crate unicode_segmentation;
// Cargo.toml has the following
// [dependencies]
// unicode-segmentation = "1.2.1"

use std::iter::FromIterator;
use unicode_segmentation::UnicodeSegmentation;

// Reverse a unicode string, observing unicode segmentation rules
pub fn reverse(input: &str) -> String {
    let mut myvec = Vec::new();
    for grapheme in UnicodeSegmentation::graphemes(input, true).rev() {
        myvec.push(grapheme);
    }
    String::from_iter(myvec)
}
