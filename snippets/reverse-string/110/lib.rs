use unicode_segmentation::UnicodeSegmentation;

// This only works for single-byte characters in utf-8!
//pub fn reverse(input: &str) -> String {
//    let mut bytes = input.as_bytes().clone().to_owned();
//    bytes.reverse();
//    let reversed = String::from_utf8(bytes.to_vec()).unwrap();
//    reversed
//}

pub fn reverse(input: &str) -> String {
    let mut gr_ind = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    gr_ind.reverse();
    gr_ind.join("")
}





