extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // Build a Vec of valid Unicode character string
    let new_vec = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();

    let mut reversed_string = String::new();
    // reverse the Vec and concat into a reversed string
    for string_bit in new_vec.iter().rev() {
        reversed_string += string_bit;
    }

    reversed_string
    //unimplemented!("Write a function to reverse {}", input);
}
