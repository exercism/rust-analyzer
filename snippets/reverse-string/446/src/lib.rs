pub fn reverse(input: &str) -> String {
    //unimplemented!("Write a function to reverse {}", input);
    let word = input.chars().rev().collect::<String>();
    return word;
}
