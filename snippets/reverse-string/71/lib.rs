extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

/// Reverses the string supplied, returning the results
///
/// # Arguments
///
/// * `input` - A string to be reversed
pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect::<String>()
}
