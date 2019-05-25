extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let reversed: String = UnicodeSegmentation::graphemes(input, true).rev().collect();
    reversed
}
