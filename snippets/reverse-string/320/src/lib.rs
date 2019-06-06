#[cfg(feature = "graphemes")]
extern crate unicode_segmentation;

#[cfg(feature = "graphemes")]
use unicode_segmentation::UnicodeSegmentation;

#[cfg(feature = "graphemes")]
pub fn reverse(input: &str) -> String {
    let mut graphemes: Vec<&str> =
        UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    graphemes.reverse();

    let reversed_string: String = graphemes.concat();
    reversed_string
}

#[cfg(not(feature = "graphemes"))]
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect::<String>()
}
