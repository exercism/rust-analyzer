#[cfg(feature = "grapheme")]
use unicode_segmentation::UnicodeSegmentation;

#[cfg(feature = "grapheme")]
pub fn reverse(input: &str) -> String {
    UnicodeSegmentation::graphemes(input, true)
        .rev()
        .fold(String::new(), |mut acc, c| {
            acc.push_str(c);
            acc
        })
}

#[cfg(not(feature = "grapheme"))]
pub fn reverse(input: &str) -> String {
    input
        .chars()
        .rev()
        .fold(String::new(), |acc, c| format!("{}{}", acc, c))
}
