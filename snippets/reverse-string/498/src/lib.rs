pub fn reverse(input: &str) -> String {

    let mut output = String::with_capacity(input.len());

    for c in input.chars().rev(){
        output.push(c);
    }
    output
}
