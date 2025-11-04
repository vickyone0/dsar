pub fn validating_pharanthasis(s: String) -> bool {
    let mut stack = Vec::new();

    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => stack.pop() != Some('('){ return false},
            ']' => stack.pop() != Some('['){ return false},
            '}' => stack.pop() != Some('{'){ return false},
            _ => return false,
        }
    }

    stack.is_empty()
}