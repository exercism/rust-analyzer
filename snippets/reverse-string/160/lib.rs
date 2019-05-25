use std::str::Graphemes;

pub fn reverse(input: &str) -> String {
    let mut input_string: String = String::from(input);
    unsafe {
        let vec = input_string.as_mut_vec();
        vec.reverse();
        let result = match String::from_utf8(vec.to_owned()) {
            Ok(value) => value,
            Err(e) => panic!("Could not convert vector ({:?}) to string: {}", vec, e),
        };
        return result;
    }
}
