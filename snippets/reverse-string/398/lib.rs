extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut reversed: String = "".to_string();
    for l in UnicodeSegmentation::graphemes(input, true).rev() {
        reversed += l;
    }

    reversed
}
