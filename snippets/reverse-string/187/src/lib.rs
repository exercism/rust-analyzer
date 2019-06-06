pub fn reverse(input: &str) -> String {
    let mut result = String::new();

    for x in input.chars().rev() {
        result.push(x);
    }

    result
}
