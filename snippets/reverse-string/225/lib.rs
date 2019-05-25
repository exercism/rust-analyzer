pub fn reverse(input: &str) -> String {
    let mut rev_str = String::new();
    rev_str.reserve(input.len());
    for c in input.chars().rev() {
        rev_str.push(c);
    }
    rev_str
}
