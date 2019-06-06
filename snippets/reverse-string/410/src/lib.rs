pub fn reverse(input: &str) -> String {

    println!("str len : {}", input.len());
    let mut result = String::with_capacity(input.len());
    let len = input.len();

    result.insert(4, 'c');
    println!("123456");

    for (i, c) in input.chars().enumerate() {
        println!("i : {}, c : {}", i, c);
    }
    return result;
}
