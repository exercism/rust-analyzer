extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let g_input = input.graphemes(true);

    g_input.rev().collect()
}
