pub fn reverse(input: &str) -> String {
    return input.chars().fold(String::new(),
                              |acc, x| format!("{}{}", x, acc));
}
