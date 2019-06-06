extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // Non-grapheme version version works for simple text:
    // input.chars().rev().collect::<String>()

    // To parse the grapheme clusters, we need to use the `unicode_segmentation` crate
    UnicodeSegmentation::graphemes(input, true).rev().collect::<String>()
}
