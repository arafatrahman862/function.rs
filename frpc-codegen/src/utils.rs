pub fn join(strings: impl Iterator<Item = String>, separator: &str) -> String {
    let mut string = String::new();
    let mut first = true;
    for s in strings {
        if first {
            first = false;
        } else {
            string.push_str(separator);
        }
        string.push_str(&s);
    }
    string
}