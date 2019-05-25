pub fn reverse(input: &str) -> String {
    input.chars().fold("".to_string(), |result, character| {
        let mut new_result = result;
        new_result.insert(0, character);
        new_result
    })
}
