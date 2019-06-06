pub fn reverse(input: &str) -> String {
    let mut output: String = "".to_owned();
    for c in input.chars().rev() {
        output.push(c)
    }
    output
}
