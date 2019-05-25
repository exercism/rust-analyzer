extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // no idea what is happening with .collect here
    let input_string = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();

    let mut reversed: String = "".to_string();
    for c in input_string {
        reversed = format!("{}{}", &c, reversed);
    }
    return reversed;

}
