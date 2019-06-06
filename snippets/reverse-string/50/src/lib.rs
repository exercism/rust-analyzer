pub fn reverse(input: &str) -> String {
    // input.chars().rev().collect() 

    let chars: Vec<char> = input.chars().collect();
    let chars_len = chars.len();
    let mut reverse_input = String::with_capacity(chars_len);

    for i in 0..chars_len {
        reverse_input.push(chars[chars_len - 1 - i]);
    }

    return reverse_input;
}


