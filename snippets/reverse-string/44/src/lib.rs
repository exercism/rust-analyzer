pub fn reverse(input: &str) -> String {
    let mut return_string = String::new();
    for c in input.chars().rev() {
        return_string.push(c);
    }
    return_string
}
