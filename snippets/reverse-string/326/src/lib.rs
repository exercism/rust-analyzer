pub fn reverse(input: &str) -> String {
    let mut result = String::from("");
    for i in input.chars().rev() {
        result.push(i);
    }

    result
}
