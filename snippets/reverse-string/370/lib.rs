pub fn reverse(input: &str) -> String {
    input.chars().rev().map(|c| c.to_string()).collect()
}
