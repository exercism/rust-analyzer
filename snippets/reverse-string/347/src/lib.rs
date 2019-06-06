extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let rev: String = input
        .graphemes(true)
        .rev()
        .flat_map(|g| g.chars())
        .collect();

    rev
}
