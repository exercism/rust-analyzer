pub fn reverse(input: &str) -> String {
    // In order to improve readability, store the input string's length
    let input_len = input.chars().count();
    // Create string object for reverse string with input string's length
    let mut reversed_string = String::with_capacity(input_len);

    // Reverse the input string by placing each charater in the newly created string
    for i in 0..input_len {
        match input.chars().nth(input_len - (i + 1)) {
            None => panic!("out of bounds"),
            Some(value) => reversed_string.push(value),
        };
    }
    return reversed_string;
}
