extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

fn reversed(s: &str) -> String {
    let r: String = s.graphemes(true).rev().collect();
    r
}