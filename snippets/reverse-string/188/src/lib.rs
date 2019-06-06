use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input
        .graphemes(true)
        .rev()
        .flat_map(|grapheme| grapheme.chars())
        .collect()
}
