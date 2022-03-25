use super::*;

impl Struct {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            fields: vec![],
        }
    }
    pub fn field(&mut self, name: impl Into<String>, ty: Type) -> &mut Self {
        self.fields.push(Field {
            name: name.into(),
            ty,
        });
        self
    }
}

impl Debug for Struct {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if !self.name.is_empty() {
            write!(f, "interface ")?;
        }
        let mut obj = f.debug_struct(&self.name);
        for field in &self.fields {
            match &field.ty {
                Type::Option(ty) => obj.field(&format!("{}?", field.name), &TypeOf(ty)),
                _ => obj.field(&field.name, &TypeOf(&field.ty)),
            };
        }
        obj.finish()
    }
}

#[test]
#[cfg(test)]
fn test() {
    let mut struct_ty = Struct::new("Test");

    struct_ty
        .field("a", Type::U16)
        .field("b", Type::U16.optional());

    assert_eq!(
        format!("{:?}", struct_ty),
        "interface Test { a: number, b?: number }"
    );
}
