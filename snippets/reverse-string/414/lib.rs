pub fn reverse(input: &str) -> String {
    let mut new_str: String = String::new();
    println!("String {} len {}", input, input.len());
    for i in input.chars().rev() {
        new_str.push(i);
    }

    new_str
}
