

pub fn reverse(input: &str) -> String {
    let mut rs: String = String::new();
    for c in input.chars().rev() {
        rs.push(c);
    }
    rs
}
