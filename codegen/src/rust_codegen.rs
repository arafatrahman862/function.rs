use super::*;
use std::fmt::{self, Debug};

struct CodeGen(TypeDef);

// impl Debug for CodeGen {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let TypeDef {
//             name,
//             version,
//             funcs,
//         } = &self.0;

//         write!(f, "trait {name} {{");
//         for Func { name, args, ret_ty } in funcs {
//             write!(f, "fn {name}() ->")?;
//         }
//         write!(f, "}}")
//     }
// }

// struct Ty(Type);

// impl Debug for Ty {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match &self.0 {
//             Type::str | Type::String => write!(f, "String"),
//             Type::Option(ty) => write!(f, "Option<{:?}>", Ty((**ty).clone())),
//             Type::Result(ty) => write!(f, "Result<{:?}, {:?}>", Ty(ty.0.clone()), Ty(ty.1.clone())),
//             Type::Slice(_) => todo!(),
//             Type::Tuple(_) => todo!(),
//             Type::TupleStruct { name, fields } => todo!(),
//             Type::Struct { name, fields } => {
//                 write!(f, "struct ")?;
//                 fields
//                     .iter()
//                     .fold(&mut f.debug_struct(name), |s, (name, ty)| {
//                         s.field(name, &Ty(ty.clone()))
//                     })
//                     .finish()
//             }
//             Type::Enum { name, fields } => {
//                 write!(f, "enum {name}")?;
//                 Ok(())
//             }
//             Type::Array { len, ty } => write!(f, "[{:?}; {len}]", Ty((**ty).clone())),
//             Type::Set { variant, ty } => todo!(),
//             Type::Map { variant, ty } => todo!(),
//             ty => write!(f, "{ty:?}"),
//         }
//     }
// }

// #[test]
// fn test_name() {
//     let s = Type::Struct {
//         name: "Foo".into(),
//         fields: Box::new([
//             ("bar".into(), Type::u16),
//             ("baz".into(), Type::char),
//             (
//                 "arr".into(),
//                 Type::Array {
//                     len: 5,
//                     ty: Box::new(Type::bool),
//                 },
//             ),
//         ]),
//     };
//     println!("{:?}", Ty(s));
// }
