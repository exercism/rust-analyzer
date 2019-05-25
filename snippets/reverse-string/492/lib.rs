extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut output = String::new();

    for g in input.graphemes(true).rev() {
        output.push_str(g);
    }
    return output;
}
