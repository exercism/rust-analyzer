pub fn reverse(input: &str) -> String {
    let mut result = String::from(""); 
    for c in input.chars().rev() {
    	result.push(c);
    }
    result
}
