pub fn reverse(input: &str) -> String {
    if input.is_empty() {
        input.to_string();
    }

    input.chars().rev().collect()
}
