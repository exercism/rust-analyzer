pub fn reverse(input: &str) -> String {
    let mut s = String::new();

    for c in input.chars() {
        s.insert(0, c);
    }

    s
}
