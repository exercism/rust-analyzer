pub fn reverse(input: &str) -> String {
    return input.chars().rev().collect::<String>();

    // ignore the following code
    // it was my first test
    if input == "" {
        return String::from("");
    } else {
        let mut result = String::from("");

        for index in 1..input.len() + 1 {
            let c = input.chars().nth(input.len() - index).unwrap();
            println!("{0} -> {1}", index, c);
            result.push(c);
        }

        return result;
    }
}

// ignore this code
fn main() {
    println!("{}", reverse("子猫"))
}
