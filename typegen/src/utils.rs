pub fn parse_angle_bracket_inner(string: &str, start: char, end: char) -> &str {
    &string[string.find(start).unwrap() + 1..string.rfind(end).unwrap()]
}

pub fn split_items_outside_group(string: &str) -> Vec<&str> {
    let mut items = Vec::new();
    let mut in_group = false;
    let mut start = 0;
    for (end, c) in string.chars().enumerate() {
        match c {
            '<' | '(' | '[' | '{' => in_group = true,
            '>' | ')' | ']' | '}' => in_group = false,
            // split at comma
            ',' if !in_group => {
                items.push(string[start..end].trim());
                start = end + 1;
            }
            _ => {}
        }
    }
    items.push(string[start..].trim());
    items
}

#[test]
fn test_split_items_outside_group() {
    assert_eq!(split_items_outside_group("a, b, c"), vec!["a", "b", "c"]);
    assert_eq!(split_items_outside_group("(a, b), c"), vec!["(a, b)", "c"]);
    assert_eq!(
        split_items_outside_group("(a, b), c, d"),
        vec!["(a, b)", "c", "d"]
    );
    assert_eq!(
        split_items_outside_group("(a, b), (c, d), (e, f)"),
        vec!["(a, b)", "(c, d)", "(e, f)"]
    );
    assert_eq!(
        split_items_outside_group("(a, b), c, d, (e, f)"),
        vec!["(a, b)", "c", "d", "(e, f)"]
    );
}