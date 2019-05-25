extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
  // input.chars().rev().collect::<String>()
  UnicodeSegmentation::graphemes(input, true)
    .rev()
    .collect::<String>()
}
