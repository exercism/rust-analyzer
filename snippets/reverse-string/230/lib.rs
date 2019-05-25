pub fn reverse(input: &str) -> String {
    //unimplemented!("Write a function to reverse {}", input);
    let mut ret = String::new();
    let mut input_string = String::from(input);
    while input_string.len() > 0 {
        ret.push(input_string.pop().unwrap());
    }
    ret
}
