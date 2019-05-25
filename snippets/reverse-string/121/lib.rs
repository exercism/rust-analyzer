pub fn reverse(input: &str) -> String {
    input.chars()
        .fold(Vec::new(), |mut acc, c| {acc.insert(0, c.to_string()); acc})
        .join("")
}
