pub fn reverse(input: &str) -> String {
    let mut reverse_input = String::new();
    for c in input.chars().rev() {
        reverse_input.push(c);
    }

    reverse_input
}
