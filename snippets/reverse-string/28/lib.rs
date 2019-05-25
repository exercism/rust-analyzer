// extern crate unicode-segmentation;

use std::iter::FromIterator;
// use unicode_segmentation::Graphemes;

pub fn reverse(input: &str) -> String {
    let tupni = String::from_iter(input.chars().rev());
    // let tupni = String::from_iter(input.graphemes(true).rev());

    return tupni; 
}
