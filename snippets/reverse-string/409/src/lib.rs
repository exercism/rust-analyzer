use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let in_str = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    let mut output = Vec::new();
    for i in in_str.iter().rev() {
        output.push(i);
    }
    output.into_iter().map(|i| i.to_string()).collect::<String>()
}

fn main() {
    println!("{}", reverse("uüu"));
}


