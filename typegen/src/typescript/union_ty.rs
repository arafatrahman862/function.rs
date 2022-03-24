use super::*;

impl Union {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            variants: vec![],
        }
    }
    pub fn variant(&mut self, name: impl Into<String>, variant: Variant) -> &mut Self {
        self.variants.push(variant);
        self
    }
}

impl Debug for Union {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "type {} = ", self.name)?;
        let fmt_str = self
            .variants
            .iter()
            .map(|v| match v {
                Variant::Unit(name) => format!("{{ type: {:?} }}", name),
                Variant::Tuple(name, types) => format!(
                    "{{ type: {:?}, value: [{}] }}",
                    name,
                    types
                        .iter()
                        .map(|ty| format!("{:?}", TypeOf(ty)))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                Variant::Named(name, fields) => format!(
                    "{{ type: {:?}, value:{:?} }}",
                    name,
                    Struct {
                        name: "".into(), // anonymous interface
                        fields: fields.to_vec()
                    }
                ),
            })
            .collect::<Vec<_>>()
            .join(if f.alternate() { "\n\t| " } else { " | " });

        f.write_str(&fmt_str)
    }
}

#[test]
fn test() {
    let mut union_ty = Union::new("Union");
    union_ty
        .variant("Unit", Variant::Unit("Unit".into()))
        .variant("Tuple", Variant::Tuple("Tuple".into(), vec![Type::I32]))
        .variant(
            "Named",
            Variant::Named(
                "Named".into(),
                vec![
                    Field {
                        name: "f".into(),
                        ty: Type::I32,
                    },
                    Field {
                        name: "g".into(),
                        ty: Type::String,
                    },
                ],
            ),
        );

    let out = r#"type Union = { type: "Unit" } | { type: "Tuple", value: [number] } | { type: "Named", value: { f: number, g: string } }"#;
    assert_eq!(format!("{:?}", union_ty), out);
}
