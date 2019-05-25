extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut v = Vec::new();
    for grapheme in UnicodeSegmentation::graphemes(input, true).rev() {
        for ch in grapheme.chars() {
            v.push(ch)
        }
    }
     v.iter().collect()
}
