pub fn reverse(input: &str) -> String {
    input.chars().fold(String::new(),|a,b|format!("{}{}",b,a))
    //unimplemented!("Write a function to reverse {}", input);
}
