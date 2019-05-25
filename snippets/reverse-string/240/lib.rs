pub fn reverse(input: &str) -> String {
    let mut reversed = String::with_capacity(input.len());
    for c in input.chars() {
        reversed.insert(0, c);
    }
    reversed
}
