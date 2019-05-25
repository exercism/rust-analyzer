use std::collections::VecDeque;

pub fn reverse(input: &str) -> String {
    let mut rstr = VecDeque::<char>::new();
    let len = input.chars().count();
    for s in input.chars() {
        rstr.push_back(s);
    }
    let mut ret: String = String::new();
    for _i in 0..len {
        ret.push(rstr.pop_back().unwrap());
    }
    ret
}
