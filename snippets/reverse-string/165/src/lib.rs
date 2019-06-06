pub fn reverse(input: &str) -> String {
    input
        .chars()
        .fold(String::with_capacity(input.len()), |mut acc, ch| {
            acc.insert(0, ch);
            acc
        })
}
