pub fn reverse(input: &str) -> String {
    let localString: String =  String::from( input ) ;
    localString.chars().rev().collect() 
}
