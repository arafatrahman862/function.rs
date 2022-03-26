use super::*;

impl Union {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            variants: vec![],
        }
    }
    pub fn variant(&mut self, variant: Variant) -> &mut Self {
        self.variants.push(variant);
        self
    }
}

struct FmtUnionNamed<'a> {
    name: &'a String,
    fields: &'a Vec<Field>,
}
impl Debug for FmtUnionNamed<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut map = f.debug_map();
        map.entry(&format_args!("type"), self.name);
        for field in self.fields {
            match &field.ty {
                Type::Option(ty) => map.entry(&format_args!("{}?", field.name), &TypeOf(ty)),
                _ => map.entry(&format_args!("{}", field.name), &TypeOf(&field.ty)),
            };
        }
        map.finish()
    }
}

impl Debug for Union {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "type {} = ", self.name)?;
        let fmt_str = self
            .variants
            .iter()
            .map(|v| match v {
                Variant::Unit(name) => format!("{{type: {:?}}}", name),
                Variant::Tuple(name, types) => {
                    let fields = &types
                        .iter()
                        .enumerate()
                        .map(|(i, ty)| Field {
                            name: i.to_string(),
                            ty: ty.clone(),
                        })
                        .collect::<Vec<_>>();

                    format!("{:?}", FmtUnionNamed { name, fields })
                }
                Variant::Named(name, fields) => {
                    format!("{:?}", FmtUnionNamed { name, fields })
                }
            })
            .collect::<Vec<_>>()
            .join(if f.alternate() { "\n\t| " } else { " | " });

        f.write_str(&fmt_str)
    }
}

#[test]
#[cfg(test)]
fn test() {
    let mut union_ty = Union::new("Union");
    union_ty
        .variant(Variant::Unit("Unit".into()))
        .variant(Variant::Tuple(
            "Tuple".into(),
            vec![Type::I32, Type::String.optional()],
        ))
        .variant(Variant::Named(
            "Named".into(),
            vec![
                Field {
                    name: "a".into(),
                    ty: Type::I32,
                },
                Field {
                    name: "b".into(),
                    ty: Type::String.optional(),
                },
            ],
        ));
    let out = r#"type Union = {type: "Unit"} | {type: "Tuple", 0: number, 1?: string} | {type: "Named", a: number, b?: string}"#;
    assert_eq!(format!("{:?}", union_ty), out);
}
