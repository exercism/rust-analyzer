pub fn reverse(input: &str) -> String {
    let a = input.len();
    let b = (1..a).map(|x| match input.chars().nth(a-x){
        Some(y) =>y,
        None    =>' ',
    }).collect::<String>();
    //let c = input.reverse();//.collect();
    //return String::from(b);
    return b;
}
