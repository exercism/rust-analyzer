use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut ostring = String::with_capacity(input.len());
    input.graphemes(true).rev().for_each(|x| ostring.push_str(x));
    ostring
}
