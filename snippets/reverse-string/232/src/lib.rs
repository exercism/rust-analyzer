pub fn reverse(input: &str) -> String {
    let target : String = String::from(input);
    let mut output : String = String::new();

    for c in target.chars().rev() { 
        output.push(c);
    }

   output
}
