pub fn reverse(input: &str) -> String {
    let mut new_string = String::new();

    for c in input.chars().rev() {
        new_string.push(c);
    }

    new_string
}
