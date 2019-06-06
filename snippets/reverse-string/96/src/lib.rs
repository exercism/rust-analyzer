pub fn reverse(input: &str) -> String {
    let mut str = String::from("");
    for c in input.chars().rev() {
        str.push(c);
    }
    str
}
