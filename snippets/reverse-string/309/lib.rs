// NOTE:
// currently running cargo test --features graphem fails
// However, you could have a look at:
//  - this crate: https://crates.io/crates/unicode-segmentation
// and / or
// - this unstable feature: https://doc.rust-lang.org/1.3.0/std/str/struct.Graphemes.html, https://doc.rust-lang.org/1.3.0/std/primitive.str.html#method.graphemes
//   However this feature also points to the crate above

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect::<String>()
}