extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    println!("{:?}", UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>());

    let graphemes = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();

    graphemes.into_iter().rev().collect()
}
