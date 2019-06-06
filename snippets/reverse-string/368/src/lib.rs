pub fn reverse(input: &str) -> String {
    let mut string = String::from("");
    for char in input.chars() {
        string = format!("{}{}", char, string);
    };

    println!("{} <-> {}", input, string);
    string
}
