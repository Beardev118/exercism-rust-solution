pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = Vec::new();

    for chr in string.chars() {
        match chr {
            '(' | '{' | '[' => brackets.push(chr),
            ')' => {
                if brackets.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if brackets.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if brackets.pop() != Some('{') {
                    return false;
                }
            }
            _ => (),
        }
    }

    brackets.is_empty()
}
