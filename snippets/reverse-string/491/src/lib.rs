pub fn reverse(input: &str) -> String {
    let mut result = String::from("");
    for cc in input.chars().rev() {
        result.push(cc);
    }
    return result

    // this does not work for unicode
    // let input_string = String::from(input);
    // let mut buff: Vec<u8> = input_string.into_bytes();
    // let len = buff.len();

    // for idx in 0..len/2 {
    //     let mut temp = buff[idx];
    //     buff[idx] = buff[len - idx - 1];
    //     buff[len - idx - 1] = temp;
    // }
    // return String::from_utf8(buff).unwrap();
}
