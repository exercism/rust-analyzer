pub fn reverse(input: &str) -> String {
    //unimplemented!("Write a function to reverse {}", input);
    let mut s = String::with_capacity(input.len());
    for ch in input.chars() {
        s.insert(0, ch);
    }
    s
}
