pub fn reverse(input: &str) -> String {
    let s = String::from(input);
    let mut r = String::new();

    for letter in s.chars().rev() {
        r.push_str(&char::to_string(letter));
    }
    return r;
}
