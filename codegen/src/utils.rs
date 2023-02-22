use std::fmt;

// Todo: Use `&Vec<String>` instade of `&str`
pub fn write_doc_comments(f: &mut impl fmt::Write, lines: &str) -> fmt::Result {
    if lines.is_empty() {
        return Ok(());
    }
    writeln!(f, "/**")?;
    for line in lines.trim().lines() {
        writeln!(f, " * {line}")?;
    }
    writeln!(f, " */")
}

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

pub fn to_camel_case(s: &str, separator: char) -> String {
    let mut out = String::new();
    let mut capitalize_next = true;
    for c in s.chars() {
        if c == separator {
            capitalize_next = true;
        } else if capitalize_next {
            capitalize_next = false;
            out.push(c.to_uppercase().next().unwrap());
        } else {
            out.push(c);
        }
    }
    out
}
