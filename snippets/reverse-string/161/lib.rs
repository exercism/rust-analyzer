extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut s = "".to_owned();
    for ch in UnicodeSegmentation::graphemes(input, true){
        s.insert_str(0, ch);
    }
    return s;
}
