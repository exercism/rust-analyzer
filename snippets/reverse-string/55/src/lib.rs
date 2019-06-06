pub fn reverse(input: &str) -> String {
    let mut s = String::with_capacity(input.len());
    for i in input.chars().rev(){
        s.push(i);
    }
    s
}
