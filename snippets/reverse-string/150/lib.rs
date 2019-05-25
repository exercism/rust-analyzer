extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
  let reversed: String = input
    .graphemes(true)
    .rev()
    .flat_map(|g| g.chars())
    .collect();
  return reversed;
}
