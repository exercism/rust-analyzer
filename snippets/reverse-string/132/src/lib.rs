extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let result: String = input.graphemes(true).rev().collect();
    result    
}
    
