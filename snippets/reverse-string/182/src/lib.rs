// Import the neccesary crate
extern crate unicode_segmentation;

// Allow app to use the crate
use unicode_segmentation::UnicodeSegmentation;

// This function reverses any unicode string input
// and returns a reverse of the string
pub fn reverse(input: &str) -> String {
    // Loop through string's grapheme clusters
    // reverse the order of occurence and 
    // return the reversed as a String collection
    input.graphemes(true).rev().collect()
}
