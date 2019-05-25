extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    //unimplemented!("Write a function to reverse {}", input);
    input
        .graphemes(true)
        .rev()
        .flat_map(|g| g.chars())
        .collect()
}
