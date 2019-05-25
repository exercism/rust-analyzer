extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut res = String::new();
    for gr in UnicodeSegmentation::graphemes(input, true) {
        res.insert_str(0, gr);
    }
    res
}
