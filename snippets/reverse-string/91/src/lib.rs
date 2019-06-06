use unicode_reverse::reverse_grapheme_clusters_in_place;

pub fn reverse(input: &str) -> String {
    let mut reversed = String::from(input);
    reverse_grapheme_clusters_in_place(&mut reversed);
    reversed
}

// This solves all but the grapheme test
// pub fn reverse(input: &str) -> String {
//     let mut reversed = String::new();
//     let iter = input.chars().rev();
//     for c in iter {
//         reversed.push(c);
//     }
//     reversed
// }
