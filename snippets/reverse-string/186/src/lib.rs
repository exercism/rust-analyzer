pub fn reverse(input: &str) -> String {
    let mut result_string = vec![];
    for char in input.chars(){
        result_string.insert(0, char.to_string());
    }
    result_string.join("")
}
