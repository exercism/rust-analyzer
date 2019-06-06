pub fn reverse(input: &str) -> String {
    let mut result = String::new();
    for char in input.chars().rev() {
        result.push(char);
    }
    result
}
