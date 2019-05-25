extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut s = String::with_capacity(input.len());

    // for c in input.chars().rev() {
    //     s.push(c);
    // }

    for c in UnicodeSegmentation::graphemes(input, true).rev() {
        s.push_str(c);
    }

    s
}
