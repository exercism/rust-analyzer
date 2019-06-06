use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut s = String::new();
    let graphemes = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    for c in graphemes.iter() {
        s.insert_str(0, c);
        println!("{}", c);
    }
    s
}
