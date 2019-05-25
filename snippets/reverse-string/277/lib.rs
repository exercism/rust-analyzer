extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let x : String = input.chars().collect();
    let out : String = x.graphemes(true).rev().collect();
    out
}
