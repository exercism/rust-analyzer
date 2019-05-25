pub fn reverse(input: &str) -> String {
    input.chars().rev().map(|x| x.to_string()).collect::<Vec<String>>().join("")
}
