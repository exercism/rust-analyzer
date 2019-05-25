use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut result = String::new();
    let mut graphemes = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    graphemes.reverse();

    for g in graphemes {
        result.push_str(g);
    }

    result
}
