pub fn reverse(input: &str) -> String {
    let mut str = String::from("");
    for c in input.chars() {
        str.insert(0, c);
    }
    str
}
