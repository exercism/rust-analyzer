extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let graphemes = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();

    let mut reversed = String::new();
    for g in graphemes.iter().rev() {
        reversed.push_str(g);
    }
    reversed
}
