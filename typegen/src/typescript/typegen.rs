use crate::*;
use Type::*;

pub struct TypeGen<'a>(pub &'a Type);

impl fmt::Debug for TypeGen<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Struct { name, fields } => {
                if !name.is_empty() {
                    write!(f, "export interface ")?;
                }
                let mut obj = f.debug_struct(name);
                for field in fields {
                    if let Option(ty) = &field.ty {
                        obj.field(&format!("{}?", field.name), &TypeOf(ty));
                    } else {
                        obj.field(&field.name, &TypeOf(&field.ty));
                    }
                }
                obj.finish()
            }
            Fn { name, args, ret } => {
                write!(f, "export function {name}(")?;

                let mut is_optional = true;
                let mut fmt_args: std::vec::Vec<_> = args
                    .iter()
                    .rev()
                    .map(|arg| match &arg.ty {
                        Option(ty) if is_optional == true => {
                            format!("{}?: {:?}", arg.name, TypeOf(ty))
                        }
                        ty => {
                            is_optional = false;
                            format!("{}: {:?}", arg.name, TypeOf(ty))
                        }
                    })
                    .collect();

                fmt_args.reverse();
                write!(f, "{})", fmt_args.join(", "))?;

                match ret.as_ref() {
                    Null => write!(f, ": void"),
                    ty => write!(f, ": {:?}", TypeOf(ty)),
                }
            }
            Union { name, variants } => {
                write!(f, "export type {} = ", name)?;
                let fmt_str = variants
                    .iter()
                    .map(|v| match v {
                        Variant::Unit(name) => format!("{{ type: {:?} }}", name),
                        Variant::Tuple(name, types) => format!(
                            "{{ type: {:?}, value: [{}] }}",
                            name,
                            types
                                .iter()
                                .map(|ty| format!("{:?}", TypeOf(ty)))
                                .collect::<std::vec::Vec<_>>()
                                .join(", ")
                        ),
                        Variant::Named(name, fields) => format!(
                            "{{ type: {:?}, value:{:?} }}",
                            name,
                            TypeGen(&Struct {
                                name: "".into(), // anonymous interface
                                fields: fields.to_vec()
                            })
                        ),
                    })
                    .collect::<std::vec::Vec<_>>()
                    .join(if f.alternate() { "\n\t| " } else { " | " });

                f.write_str(&fmt_str)
            }
            Enum { field, entries } => todo!(),

            ty => TypeOf(ty).fmt(f),
        }
    }
}

struct TypeOf<'a>(pub &'a Type);

impl<'a> fmt::Debug for TypeOf<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Struct { name, fields } => f.write_str(name),
            Fn { name, args, ret } => f.write_str(name),
            Union { name, variants } => f.write_str(name),
            Enum { field, entries } => f.write_str(&field.name),

            Any => f.write_str("any"),
            Null => f.write_str("null"),
            Bool => f.write_str("boolean"),
            String => f.write_str("string"),
            U8 | U16 | U32 | I8 | I16 | I32 | F32 | F64 => f.write_str("number"),
            I64 | U64 | I128 | U128 => f.write_str("bigint"),

            Vec(ty) => return write!(f, "Array<{:?}>", TypeOf(ty)),
            Option(ty) => return write!(f, "{:?} | undefiend", TypeGen(ty)),
            Result(ty, _) => return TypeOf(ty).fmt(f),
        }
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;

    #[test]
    fn test_union() {
        let union_ty = Union {
            name: "Foo".into(),
            variants: vec![
                Variant::Unit("Bar".into()),
                Variant::Tuple("Baz".into(), vec![U8, Any]),
                Variant::Named("Qux".into(), vec![
                    Field { name: "a".into(), ty: U8 },
                    Field { name: "b".into(), ty: Option(Box::new(U16)) },
                ]),
            ],
        };
        let out = r#"export type Foo = { type: "Bar" } | { type: "Baz", value: [number, any] } | { type: "Qux", value: { a: number, b?: number } }"#;
        assert_eq!(out, format!("{:?}", TypeGen(&union_ty)));
    }

    #[test]
    fn test_function() {
        let struct_type = Fn {
            name: "add".to_string(),
            args: vec![
                Field { name: "x".into(), ty: U16 },
                Field { name: "y".into(), ty: Option(Box::new(U16)) },
                Field { name: "y".into(), ty: Option(Box::new(U32)) },
            ],
            ret: Box::new(Option(Box::new(Null))),
        };
        println!("\n{:?}\n", TypeGen(&struct_type));
    }
    #[test]
    fn test_interface() {
        let struct_type = Struct {
            name: "Foo".into(),
            fields: vec![
                Field { name: "x".into(), ty: U16 },
                Field { name: "y".into(), ty: Option(Box::new(U16)) },
            ],
        };
        println!("{:#?}\n", TypeGen(&struct_type));
    }
}
