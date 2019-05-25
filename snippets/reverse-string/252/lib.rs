// unicode segmentation library is needed to segment grapheme clusters
use unicode_segmentation::UnicodeSegmentation;
use std::iter::FromIterator;

pub fn reverse(input: &str) -> String {
    let splitted = UnicodeSegmentation::graphemes(input, true);
    String::from_iter(splitted.rev())
}
