// extern crate unicode_segmentation;
// use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let reverse = input.chars().rev().collect::<String>();
    ("Write a function to reverse {}", reverse);
}
