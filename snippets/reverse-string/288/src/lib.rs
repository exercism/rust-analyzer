pub fn reverse(input: &str) -> String {
    let mut r = String::new();
    let mut temp = String::from(input);

    for i in 0..input.len() {
        r.push(temp.pop().unwrap());
    }
    r
}
