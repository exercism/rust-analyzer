pub fn reverse(input: &str) -> String {
    let mut s = String::from(input);
    let mut result = String::new();
    for _ in 0..s.len() {
        if let Some(c) = s.pop() {
            result.push(c)
        }
    }

    result
}
