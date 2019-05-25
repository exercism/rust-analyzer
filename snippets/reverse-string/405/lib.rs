extern crate unicode_segmentation;

use std::string::String;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let s = input.graphemes(false).rev().collect();
    println!("{}", s);
    return s;
}
