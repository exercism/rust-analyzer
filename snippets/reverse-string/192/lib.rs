pub fn reverse(input: &str) -> String {
    let mut s: String = "".to_string();

    for c in input.chars() {
        s.insert(0, c);
    }
    
    return s;
}
