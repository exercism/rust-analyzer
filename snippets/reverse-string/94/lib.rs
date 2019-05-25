pub fn reverse(input: &str) -> String {
    let length = input.len();
    let mut string = String::with_capacity(length);
    for c in input.chars() {
        string.insert(0, c);
    }
    string
}
