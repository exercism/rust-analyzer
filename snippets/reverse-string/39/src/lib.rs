extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;
use std::iter::FromIterator;

pub fn reverse(input: &str) -> String {
    let mut g = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    g.reverse();
    return String::from_iter(g);
}