pub fn reverse(input: &str) -> String {
    //unimplemented!("Write a function to reverse {}", input);

    let mut reversed_string = String::new();
    let chars : Vec<char>= input.chars().collect();
    let mut count = chars.len();
    if  !input.is_empty() {
        while count > 0 {
            reversed_string.push(chars[count - 1]);
            count -= 1;
        }
    }
    reversed_string
}

