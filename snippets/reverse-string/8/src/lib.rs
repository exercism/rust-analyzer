extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    ////works for everything    
    UnicodeSegmentation::graphemes(input, true).rev().collect()
}

pub fn no_extern_crate_reverse(input: &str) -> String {
    //works for everything but grapheme test    
    input.chars().rev().collect()
}