extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let graphemes = UnicodeSegmentation::graphemes(input, true);
    let mut graphemes_vec = graphemes.collect::<Vec<&str>>();
    graphemes_vec.reverse();
    return graphemes_vec.join("");
}
