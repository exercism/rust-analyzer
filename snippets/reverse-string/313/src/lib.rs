extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation as uniseg;

pub fn reverse(input: &str) -> String {
    uniseg::graphemes(input, true).rev().collect::<String>()
}
