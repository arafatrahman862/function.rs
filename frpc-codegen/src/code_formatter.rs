use std::fmt::Write;

pub struct CodeFormatter {
    pub buf: String,
    pub spaces: &'static str,
    pub indent_lvl: usize,
}

impl Write for CodeFormatter {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        match s {
            "{" => self.indent_lvl += 1,
            "}" => self.indent_lvl -= 1,
            _ => {}
        }
        self.buf += s;
        Ok(())
    }
}

impl CodeFormatter {
    pub fn new() -> Self {
        Self {
            buf: String::new(),
            spaces: "\t",
            indent_lvl: 0,
        }
    }

    // Todo: Use `&Vec<String>` instade of `&str`
    pub fn write_doc_comments(&mut self, lines: &str) -> std::fmt::Result {
        if lines.is_empty() {
            return Ok(());
        }
        writeln!(self, "/**\n")?;
        for line in lines.trim().lines() {
            writeln!(self, " * {line}\n")?;
        }
        writeln!(self, " */\n")
    }
}
