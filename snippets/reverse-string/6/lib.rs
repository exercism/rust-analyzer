pub fn reverse(input: &str) -> String {
    let mut s = String::new();

    for i in input.chars().rev() {
        println!("{}", i);
        s.push(i);
    }

    s
}
