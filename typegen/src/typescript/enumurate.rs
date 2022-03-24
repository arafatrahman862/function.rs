use crate::*;

impl Enum {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            entries: vec![],
        }
    }
    pub fn entry(&mut self, name: impl Into<String>, value: impl ToString) -> &mut Self {
        self.entries.push((name.into(), value.to_string()));
        self
    }
}

struct FmtEnumItem((String, String));
impl Debug for FmtEnumItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let (name, value) = &self.0;
        if value.is_empty() {
            write!(f, "{}", name)
        } else {
            write!(f, "{} = {}", name, value)
        }
    }
}

impl Debug for Enum {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "enum {} ", self.name)?;
        f.debug_set()
            .entries(self.entries.clone().into_iter().map(FmtEnumItem))
            .finish()
    }
}

#[test]
#[cfg(test)]
fn test() {
    let mut enum_ty = Enum::new("MyEnum");
    enum_ty
        .entry("A", 1)
        .entry("B", "")
        .entry("C", 3)
        .entry("D", "\"Hello, World!\"");

    assert_eq!(
        format!("{:?}", enum_ty),
        r#"enum MyEnum {A = 1, B, C = 3, D = "Hello, World!"}"#
    );
}
