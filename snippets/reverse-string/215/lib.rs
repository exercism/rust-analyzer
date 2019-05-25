extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut ret = String::new();
    if input.is_empty() {
        return ret;
    }

    let grapheme_conv = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    let length = grapheme_conv.len();

    for i in (0..length).rev() {
        ret.push_str(grapheme_conv[i]);
    }
    ret
}
