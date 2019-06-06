use std::io;
fn main() {
    let mut mystring=String::new();

    let mut anstring=String::new();

    io::stdin().read_line(&mut mystring).expect("");

    for ch in mystring.chars().rev() {
        anstring.push(ch);
    }

    println!("{}",anstring);
}