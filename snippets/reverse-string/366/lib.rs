pub fn reverse(input: &str) -> String {
    let mut vec = Vec::new();
    for c in input.chars() {
      vec.push(c);
    }
    let mut reversed = String::new();
    while let Some(popped) = vec.pop() {
      reversed.push(popped);
    }

    return reversed;
}
