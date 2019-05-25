extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    //    input.chars().rev().collect::<String>()
    input.graphemes(true).rev().flat_map(|c| c.chars()).collect()
}
