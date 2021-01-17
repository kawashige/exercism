use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let open_bracket = vec![(']', '['), ('}', '{'), (')', '(')]
        .into_iter()
        .collect::<HashMap<char, char>>();
    let mut stack = Vec::new();
    for b in string.chars() {
        match b {
            '[' | '{' | '(' => {
                stack.push(b);
            }
            ']' | '}' | ')' => {
                if stack.is_empty() || stack.pop().unwrap() != open_bracket[&b] {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}
