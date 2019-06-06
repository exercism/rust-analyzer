pub fn reverse(input: &str) -> String {
    let mut reversed = String::new();

    for c in input.chars() {
        reversed.insert(0, c)
    }
    
    reversed
}
