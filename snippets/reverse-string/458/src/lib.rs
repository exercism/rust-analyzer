pub fn reverse(input: &str) -> String {
    let mut out = String::from("");
    let mut in_str = String::from(input);
    while out.len() < input.len() {
      let last = in_str.pop();
      match last {
        Some(last) => {
          out.push(last)
        },
        None => {}
      }
    }
    out.to_string()
}
