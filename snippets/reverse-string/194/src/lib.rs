pub fn reverse(input: &str) -> String {
    input
        .chars()
        .fold(String::new(), |acc, c| format!("{}{}", c, acc))
}
