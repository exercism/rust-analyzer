pub fn reverse(input: &str) -> String {
    let mut reversed = String::with_capacity(input.len());
    for ch in input.chars() {
        reversed.insert(0, ch);
    }
    reversed
}
