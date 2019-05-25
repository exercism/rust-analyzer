use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true) // Get an iterator over the graphemes in the input
        .rev() // Reverse the iterator so we see each grapheme in reverse order
        .flat_map(|x| x.chars()) // Flat map each grapheme cluster into iterators 
        .collect() // collect everything and return it
}
