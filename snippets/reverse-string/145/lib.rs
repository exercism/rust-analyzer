pub fn reverse(input: &str) -> String {
    let len = input.len();
    let mut result = String::with_capacity(len);
    for (i, ch) in input.chars().enumerate() {
        result.insert(len-1-i, ch);
    }
    result 
}
