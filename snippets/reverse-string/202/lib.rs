pub fn reverse(input: &str) -> String {
    let mut res = String::new();
    for ch in input.chars().rev() {
        res.push(ch);
    }

    res
}
