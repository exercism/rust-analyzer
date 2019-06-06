pub fn reverse(input: &str) -> String {
    let mut array: Vec<char> = input.chars().collect();
    array.reverse();
    let mut result = String::new();
    for i in 0..array.len() {
        result.push(array[i])
    }
    result
}
