use crate::utils::join;
use crate::writer::Writer;
use frpc_message::*;
use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Write;

const _PRELUDE: &str = r#"
type Result<T, E> =
    | { ok: T }
    | { err: E }
"#;

pub fn generate(w: &mut Writer, type_def: &TypeDef) -> Result {
    for (name, kind) in &type_def.ctx.costom_types {
        let name = name.replace("::", "_");
        match kind {
            CustomTypeKind::Unit(unit) => {
                w.write_doc_comments(&unit.doc)?;

                write!(w, "enum {name} ")?;
                write_map(w, unit.fields.iter().map(|f| (&f.doc, &f.name, f.value)))?;
            }
            CustomTypeKind::Struct(data) => {
                w.write_doc_comments(&data.doc)?;

                write!(w, "interface {name} ")?;
                let fields = data.fields.iter().map(|f| (&f.doc, &f.name, ty_str(&f.ty)));
                write_map(w, fields)?;
            }
            CustomTypeKind::TupleStruct(data) => {
                w.write_doc_comments(&data.doc)?;
                let fields = join(data.fields.iter().map(|f| ty_str(&f.ty)), " ,");
                write!(w, "type {name} = [{fields}];")?;
            }
            CustomTypeKind::Enum(data) => {
                w.write_doc_comments(&data.doc)?;

                writeln!(w, "type {name} =")?;
                w.indent_lvl += 1;

                for EnumField { doc: _, name, kind } in &data.fields {
                    let fields = match kind {
                        UnionKind::Unit => format!(""),
                        UnionKind::Struct(dta) => join(
                            dta.iter().map(|f| format!("{}: {}", f.name, ty_str(&f.ty))),
                            ", ",
                        ),
                        UnionKind::Tuple(data) => join(
                            data.iter()
                                .enumerate()
                                .map(|(i, field)| format!("{i}: {}", ty_str(&field.ty))),
                            " ,",
                        ),
                    };
                    writeln!(w, "| {{ type: {name:?}, {fields}}}")?;
                }

                w.indent_lvl -= 1;
            }
        }
    }
    for Func {
        index: _,
        name,
        args,
        retn,
    } in &type_def.funcs
    {
        let name = name.replace("::", "_");
        let args = join(args.iter().map(ty_str), " ,");
        writeln!(w, "function {name}({args}): {}", ty_str(retn))?;
    }
    Ok(())
}

fn write_map<'a, I, K, V>(w: &mut Writer, fields: I) -> Result
where
    K: Display,
    V: Display,
    I: Iterator<Item = (&'a String, K, V)>,
{
    w.write_str("{\n")?;
    w.indent_lvl += 1;

    for (doc, name, item) in fields {
        w.write_doc_comments(doc)?;
        writeln!(w, "{name}: {item},")?;
    }

    w.write_str("}\n")?;
    w.indent_lvl -= 1;
    Ok(())
}

fn ty_str(ty: &Ty) -> String {
    match ty {
        Ty::u8 | Ty::u16 | Ty::u32 | Ty::i8 | Ty::i16 | Ty::i32 | Ty::f32 | Ty::f64 => {
            "number".into()
        }
        Ty::u64 | Ty::u128 | Ty::i64 | Ty::i128 => "bigint".into(),

        Ty::bool => "bool".into(),

        Ty::char | Ty::String => "string".into(),

        Ty::Option(ty) => format!("{} | null", ty_str(ty)),
        Ty::Result(ty) => format!("Result<{}, {}>", ty_str(&ty.0), ty_str(&ty.1)),

        Ty::Tuple(tys) => format!("[{}]", join(tys.iter().map(ty_str), " ,")),

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
        Ty::Map { ty, .. } => format!("Map<{}, {}>", ty_str(&ty.0), ty_str(&ty.1)),
        Ty::CustomType(ty) => ty.replace("::", "_"),
    }
}
