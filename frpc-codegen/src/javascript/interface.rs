use crate::code_formatter::write_doc_comments;
use crate::utils::{join, to_camel_case};
use frpc_message::*;
use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Write;

pub fn generate(w: &mut impl Write, type_def: &TypeDef) -> Result {
    for Func {
        index: _,
        path,
        args,
        retn,
    } in &type_def.funcs
    {
        let name = to_camel_case(path, ':');
        let args = join(args.iter().map(ty_str), ", ");
        writeln!(w, "function {name}({args}): {}", ty_str(retn))?;
    }
    Ok(())
}

pub fn gen_type(f: &mut impl Write, ident: String, kind: &CustomTypeKind) -> Result {
    Ok(match kind {
        CustomTypeKind::Unit(unit) => {
            write_doc_comments(f, &unit.doc)?;

            write!(f, "export enum {ident} ")?;
            write_map(
                f,
                " =",
                unit.fields.iter().map(|f| (&f.doc, &f.name, f.value)),
            )?;
        }
        CustomTypeKind::Struct(data) => {
            write_doc_comments(f, &data.doc)?;

            write!(f, "export interface {ident} ")?;
            let fields = data.fields.iter().map(|f| (&f.doc, &f.name, ty_str(&f.ty)));
            write_map(f, ":", fields)?;
        }
        CustomTypeKind::Tuple(data) => {
            write_doc_comments(f, &data.doc)?;
            let fields = join(data.fields.iter().map(|f| ty_str(&f.ty)), ", ");
            write!(f, "export type {ident} = [{fields}];")?;
        }
        CustomTypeKind::Enum(data) => {
            write_doc_comments(f, &data.doc)?;

            writeln!(f, "export type {ident} =")?;

            for EnumField { doc: _, name, kind } in &data.fields {
                let fields = match kind {
                    EnumKind::Unit => String::new(),
                    EnumKind::Struct(dta) => join(
                        dta.iter().map(|f| format!("{}: {}", f.name, ty_str(&f.ty))),
                        ", ",
                    ),
                    EnumKind::Tuple(data) => join(
                        data.iter()
                            .enumerate()
                            .map(|(i, field)| format!("{i}: {}", ty_str(&field.ty))),
                        ", ",
                    ),
                };
                writeln!(f, "| {{ type: {name:?}, {fields}}}")?;
            }
        }
    })
}

fn write_map<'a, I, K, V>(f: &mut impl Write, sep: &str, fields: I) -> Result
where
    K: Display,
    V: Display,
    I: Iterator<Item = (&'a String, K, V)>,
{
    writeln!(f, "{{")?;
    for (doc, name, item) in fields {
        write_doc_comments(f, doc)?;
        writeln!(f, "{name}{sep} {item},")?;
    }
    writeln!(f, "}}")
}

pub fn ty_str(ty: &Ty) -> String {
    match ty {
        Ty::u8 | Ty::u16 | Ty::u32 | Ty::i8 | Ty::i16 | Ty::i32 | Ty::f32 | Ty::f64 => {
            "number".into()
        }
        Ty::u64 | Ty::u128 | Ty::i64 | Ty::i128 => "bigint".into(),

        Ty::bool => "bool".into(),

        Ty::char | Ty::String => "string".into(),

        Ty::Array { ty, .. } | Ty::Set { ty, .. } => match **ty {
            Ty::u8 => "Uint8Array",
            Ty::u16 => "Uint16Array",
            Ty::u32 => "Uint32Array",
            Ty::u64 => "BigUint64Array",

            Ty::i8 => "Int8Array",
            Ty::i16 => "Int16Array",
            Ty::i32 => "Int32Array",
            Ty::i64 => "BigInt64Array",

            Ty::f32 => "Float32Array",
            Ty::f64 => "Float64Array",

            Ty::u128 | Ty::i128 => "Array<bigint>",
            _ => return format!("Array<{}>", ty_str(ty)),
        }
        .to_string(),

        Ty::Option(ty) => format!("{} | null", ty_str(ty)),
        Ty::Result(ty) => format!("Result<{}, {}>", ty_str(&ty.0), ty_str(&ty.1)),

        Ty::Map { ty, .. } => format!("Map<{}, {}>", ty_str(&ty.0), ty_str(&ty.1)),
        Ty::Tuple(tys) => format!("[{}]", join(tys.iter().map(ty_str), ", ")),
        Ty::CustomType(path) => to_camel_case(path, ':'),
    }
}
