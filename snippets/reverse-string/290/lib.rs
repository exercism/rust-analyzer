pub fn reverse(input: &str) -> String {
     if input == ""{return "".into()}
     else{
          let xcb =  input.to_string();
          let l = xcb.len();
          let mut end: String = "".into();
          for j in (0..l).rev(){
          end.push(xcb.chars().nth(j).expect("message"));
          };
     return end;}

}
