pub fn reverse(input: &str) -> String {
    // unimplemented!("Write a function to reverse {}", input);
    let x: String = input.to_string();
    let mut r: String = String::from("");
    for letter in x.chars().rev(){
        r.push(letter);
    }
    r
}
