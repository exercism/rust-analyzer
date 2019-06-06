pub fn reverse(input: &str) -> String {
    // unimplemented!("Write a function to reverse {}", input);
    let v_char: Vec<char> = input.chars().collect();
    let mut rev_char = String::new();
    for i in (0..v_char.len()).rev() {
        rev_char.push(v_char[i]);
    }
    rev_char
}
