use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut string = String::with_capacity(input.len());
    for s in UnicodeSegmentation::graphemes(input, true).rev() {
        string.push_str(s);
    }
    string
}
