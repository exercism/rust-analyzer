// have to import unicode_segmentation to use graphemes
extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // graphemes are user-perceived characters
    input
        // get grapheme iterator from strings
        .graphemes(true)
        // reverse the iterator
        .rev()
        // collect back into a string
        .collect()
}
