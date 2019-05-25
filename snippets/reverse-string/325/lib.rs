extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    UnicodeSegmentation::graphemes(input, true).rev().collect()
}

pub fn reverse_simple(input: &str) -> String {
    input.chars().rev().collect()
}


