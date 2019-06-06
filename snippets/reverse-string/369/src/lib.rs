
pub fn reverse(input: &str) -> String {
    let s = input.to_string();
    let mut result = String::new();
    for c in s.chars().rev(){
        result.push_str(&c.to_string());
    }
    result
}
