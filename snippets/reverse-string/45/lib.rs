pub fn reverse(input: &str) -> String {
    let mut rv = String::new();

    for c in input.chars().rev() {
        rv.push(c);
    }

    rv
}
