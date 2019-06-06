extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let char_length = input.chars().count();
    let mut reversed = String::with_capacity(char_length);

    for (_,s) in UnicodeSegmentation::graphemes(input, true).rev().enumerate() {
        reversed.push_str(s);
    }

    reversed
}