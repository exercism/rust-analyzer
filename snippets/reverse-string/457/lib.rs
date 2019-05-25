extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut stack = Vec::new();
    for c in UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>() {
        stack.push(c);
    }
    let mut s = String::new();
    for _ in 0..(stack.len()) {
        s.push_str(stack.pop().unwrap())
    }
    return s;
}
