use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let g = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    let mut s = "".to_string();
    for curr in g.iter().rev() {
        s.push_str(curr);
    }
    s
}
