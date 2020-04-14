pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();

    for c in string.chars() {
        let valid = match c {
            '{' | '[' | '(' => {
                stack.push(c);
                true
            }
            '}' => stack.pop() == Some('{'),
            ']' => stack.pop() == Some('['),
            ')' => stack.pop() == Some('('),
            _ => true,
        };

        if !valid {
            return false;
        }
    }

    stack.is_empty()
}
