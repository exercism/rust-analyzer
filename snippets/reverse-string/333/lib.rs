pub fn reverse(input: &str) -> String {
    let mut out = String::from("");
    for c in input.chars().rev() {
        out.push(c);
    }
    out
}
