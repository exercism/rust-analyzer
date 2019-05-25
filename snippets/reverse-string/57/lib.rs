extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let original_string = String::from(input);
    let mut reversed_string = Vec::new();
    let mut original_string = original_string.graphemes(true).collect::<Vec<&str>>();

    while !original_string.is_empty() {
        reversed_string.push(original_string.pop().unwrap());
    }

    reversed_string.into_iter().collect()
}
