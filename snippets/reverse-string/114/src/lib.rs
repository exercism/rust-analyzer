pub fn reverse1(input: &str) -> String {
    let mut s:String=String::from("");
    for i in (0..input.len()).rev() {let c =input.chars().nth(i);
                                     if c!=None {s.push(c.unwrap())}
                                      }
    s
}
pub fn reverse(input: &str) -> String {
    let mut s:String=String::from("");
    let mut chars=input.chars();
    let mut c=chars.next_back();
    loop{
        match c{
                Some(x) => s.push(x),
                None => break,
        }
       c=chars.next_back();
    }
    s
}
