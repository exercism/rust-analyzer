pub fn reverse(input: &str) -> String {
    let mut result = String::new();
    let input = String::from(input);
    for c in input.chars() {
        if c >= '\u{300}' && c <= '\u{36F}' {
            result.insert(1, c)
        } else {
            result.insert(0, c);
        }
    }
    result
}
