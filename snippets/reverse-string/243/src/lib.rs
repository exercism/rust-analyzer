extern crate itertools;
extern crate unicode_segmentation;

use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
  UnicodeSegmentation::graphemes(input, true).rev().join("")
}
