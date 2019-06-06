pub fn reverse(input: &str) -> String {
    let mut s = String::new();
    s = input.chars().rev().collect();
    s
}
