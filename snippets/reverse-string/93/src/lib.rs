pub fn reverse(input: &str) -> String {
    let mut reversed = String::new();

    for char in input.chars().rev() {
        reversed.push(char);
    }
    return reversed;
}
