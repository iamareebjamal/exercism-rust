pub fn reverse(input: &str) -> String {
    let mut string = String::new();

    for c in input.chars().rev() {
        string.push(c);
    }

    string
}
