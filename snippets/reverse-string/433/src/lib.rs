pub fn reverse(input: &str) -> String {
    let mut output = String::new();
    for i in input.chars() {
        output.insert(0, i);
    }
    output
    //unimplemented!("Write a function to reverse {}", input);
}
