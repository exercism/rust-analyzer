pub fn reverse(input: &str) -> String {
    let len = input.len();
    let mut myStr = String::from("");

    for i in input.chars().rev() {
        myStr.push(i);
    }
    println!("{} -> {}", input, &myStr);
    myStr
}
