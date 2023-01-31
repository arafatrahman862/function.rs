use std::fmt::Write;

pub struct CodeFormatter {
    pub buf: String,
    pub spaces: &'static str,
    pub indent_lvl: usize,
}

impl Default for CodeFormatter {
    fn default() -> Self {
        Self {
            buf: String::new(),
            spaces: "\t",
            indent_lvl: 0,
        }
    }
}

impl Write for CodeFormatter {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buf += s;
        Ok(())
    }
}

// Todo: Use `&Vec<String>` instade of `&str`
pub fn write_doc_comments(this: &mut impl Write, lines: &str) -> std::fmt::Result {
    if lines.is_empty() {
        return Ok(());
    }
    writeln!(this, "/**\n")?;
    for line in lines.trim().lines() {
        writeln!(this, " * {line}\n")?;
    }
    writeln!(this, " */\n")
}
