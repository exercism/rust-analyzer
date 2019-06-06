extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut s = String::default();
    let i = UnicodeSegmentation::graphemes(input, true);
    for g in i {
        s = String::from(g) + &s;
    }
    s
}
