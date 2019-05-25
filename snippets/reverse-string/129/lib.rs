extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    return UnicodeSegmentation::graphemes(input, true).rev().collect()
}
