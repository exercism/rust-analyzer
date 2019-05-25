extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;


pub fn reverse(input: &str) -> String {
    reverse_graphemes(input)
}

fn reverse_simple(input: &str) -> String {
    let mut result = String::new();

    for c in input.chars().rev() {
        result.push(c);
    }
    result
}

fn reverse_graphemes(input: &str) -> String {
    let mut result = String::new();

    for s in UnicodeSegmentation::graphemes(input, true).rev() {
        result.push_str(s);
    }
    result
}
