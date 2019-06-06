use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    String::from(input).graphemes(true).rev().collect()
}
