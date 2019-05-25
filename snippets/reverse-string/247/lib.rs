extern crate unicode_segmentation;

#[cfg(not(feature = "grapheme"))]
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

#[cfg(feature = "grapheme")]
pub fn reverse(input: &str) -> String {
    use unicode_segmentation::UnicodeSegmentation;

    UnicodeSegmentation::graphemes(input, true).rev().collect()
}
