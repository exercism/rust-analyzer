use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut reverse = String::new();
    for s in input.graphemes(true).rev() {
        reverse.push_str(s);
    }
    reverse
}
