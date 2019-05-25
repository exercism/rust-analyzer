extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // Solution of my own:
    // ----------
    // let s = String::from(input);
    // let mut bytes = s.into_bytes();
    // bytes.reverse();
    // String::from_utf8(bytes).unwrap()

    // Solution taken from
    // https://www.programming-idioms.org/idiom/41/reverse-a-string/402/rust
    // (fails "grapheme" test):
    // ----------
    // input.chars().rev().collect()

    UnicodeSegmentation::graphemes(input, true).rev().collect()
}
