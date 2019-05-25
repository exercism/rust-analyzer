pub fn reverse(input: &str) -> String {
    if input == "" {
        "".to_string()
    } else {
        input.chars().rev().collect::<String>()
    }
}
