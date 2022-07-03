pub fn pretty_name(input: &String) -> String {
    let mut start_of_word = true;
    input
        .chars()
        .map(|c| match c {
            '_' => {
                start_of_word = true;
                ' '
            }
            'A'..='Z' => {
                if start_of_word {
                    start_of_word = false;
                    c
                } else {
                    c.to_ascii_lowercase()
                }
            }
            _ => c,
        })
        .collect()
}
