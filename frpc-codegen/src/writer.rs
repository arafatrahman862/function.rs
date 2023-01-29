use std::fmt::Write;

pub struct Writer {
    pub buf: String,
    pub spaces: usize,
    pub indent_lvl: usize,
}

impl Writer {
    pub fn new() -> Self {
        Self {
            buf: String::new(),
            spaces: 2,
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

impl Write for Writer {
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
