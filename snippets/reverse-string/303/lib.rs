pub fn reverse(input: &str) -> String {

    let mut s: String = String::with_capacity(input.len());
    
    for c in input.chars().rev(){
        println!("This is char: {}", c);
        s.push(c);
    }
    s
}
