extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
  // I don't quite understand what the 'true' is doing...  :(
  UnicodeSegmentation::graphemes(input, true).rev().collect()
}

