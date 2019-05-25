extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let g = UnicodeSegmentation::graphemes(input, true);
    let mut reversed = String::new();

    for c in g.rev() {
        reversed.push_str(c);
    }
    reversed
}
