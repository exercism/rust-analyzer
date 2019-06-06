use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let input_graph = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    let mut reverse_input: Vec<&str> = Vec::new();
    for item in input_graph.iter().rev() {
        reverse_input.push(item);
    }
    let s: String = reverse_input.into_iter().collect();
    s
}
