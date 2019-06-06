pub fn reverse(input: &str) -> String {
    let mut result: Vec<char> = Vec::new();
    let input_vector: Vec<char> = input.chars().collect();

    for character in input_vector.iter().rev() {
        result.push(*character);
    }

    let s: String = result.into_iter().collect();
    return s;
}
