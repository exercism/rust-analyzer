// Reverse a string--e.g. "cool" as input would return "looc"
pub fn reverse(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.reverse();
    chars.iter().collect::<String>()
}
