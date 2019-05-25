pub fn reverse(input: &str) -> String {
    // extract characters from input to vector
    let mut v: Vec<char> = Vec::with_capacity(input.len());
    let s = input.to_string();
    for c in s.chars() {
        v.push(c);
    }

    // reverse characters in vector
    v.reverse();

    // build new string from reversed vector
    let mut res = String::with_capacity(v.len());
    for c in v {
        res.push(c);
    }
    res // return newly built string
}
