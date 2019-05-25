pub fn reverse(input: &str) -> String {
    let mut buffer = String::new();
    for elem in input.chars().rev() {
        buffer.push(elem);
    }
    buffer
}
