pub fn reverse(input: &str) -> String {
    // unimplemented!("Write a function to reverse {}", input);
    let mut result = String::from("");
    for c in input.chars() {
        result.insert(0, c);
    }
    return result;
}
