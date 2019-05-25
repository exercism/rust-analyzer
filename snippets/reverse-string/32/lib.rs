pub fn reverse(input: &str) -> String {
    let mut result = String::from("");
    input.chars().rev().for_each(|char| result.push(char));
    result
}
