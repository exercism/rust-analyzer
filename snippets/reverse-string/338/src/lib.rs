pub fn reverse(input: &str) -> String {
    input.chars()  // turn str into iterator of chars
         .rev()    // reverses iterator
         .collect::<String>()  // reduce the iterator values into String
}
