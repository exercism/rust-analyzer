pub fn reverse(input: &str) -> String {
    let mut chars = input.chars();
    let mut result = String::new();
    while let Some(c) = chars.next_back() {
        result.push(c);
    }
    result
}
