extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;
pub fn reverse(input: &str) -> String {
    if input.is_empty() {
        return String::from(input);
    }
    let mut stack = vec![];
    for c in UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>() { 
        stack.push(String::from(c));
    }
    let mut f = String::new();
    while let Some(c) = stack.pop() {
        f += &c;
    }
    return f;
}
