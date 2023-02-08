mod interface_path;

use super::interface::gen_type;
use crate::{
    code_formatter::write_doc_comments,
    fmt,
    utils::{join, to_camel_case},
};
use frpc_message::*;
use interface_path::InterfacePath;
use std::fmt::{Result, Write};

fn gen_input_encoders(f: &mut impl Write, type_def: &TypeDef) -> Result {
    let mut input_tys = InterfacePath::default();
    input_tys.add_tys(
        type_def.funcs.iter().flat_map(|func| func.args.iter()),
        &type_def.ctx,
    );

    for path in &input_tys.paths {
        let ident = to_camel_case(path, ':');
        let kind = &type_def.ctx.costom_types[path.clone()];
        gen_type(f, ident, kind)?;
    }

    writeln!(f, "const extern = {{")?;

    for path in input_tys.paths {
        let ident = to_camel_case(path, ':');
        writeln!(f, "{ident}(d: use.BufWriter, z: {ident}) {{")?;
        match &type_def.ctx.costom_types[path] {
            CustomTypeKind::Unit(data) => {
                writeln!(f, "switch (z) {{")?;
                for (i, UnitField { name, .. }) in data.fields.iter().enumerate() {
                    writeln!(f, "case {ident}.{name}: return d.len_u15({i});")?;
                }
                writeln!(f, "}}")?;
            }
            CustomTypeKind::Enum(data) => {
                writeln!(f, "switch (z.type) {{")?;
                for (i, EnumField { name, kind, .. }) in data.fields.iter().enumerate() {
                    writeln!(f, "case {name:?}: d.len_u15({i});")?;
                    match kind {
                        EnumKind::Struct(fields) => write_encoder_struct(f, fields)?,
                        EnumKind::Tuple(fields) => {
                            for (i, TupleField { ty, .. }) in fields.iter().enumerate() {
                                writeln!(f, "{}(z[{i}]);", field_ty(ty))?;
                            }
                        }
                        EnumKind::Unit => {}
                    }
                    writeln!(f, "break;")?;
                }
                writeln!(f, "}}")?;
            }
            CustomTypeKind::Struct(data) => write_encoder_struct(f, &data.fields)?,
            CustomTypeKind::Tuple(data) => {
                let tys = data.fields.iter().map(|f| field_ty(&f.ty));
                writeln!(f, "return d.tuple({})(z);", join(tys, ", "))?;
            }
        }
        writeln!(f, "}},")?;
    }

    writeln!(f, "}}")?;
    Ok(())
}

fn write_encoder_struct(f: &mut impl Write, fields: &Vec<StructField>) -> Result {
    fields.iter().try_for_each(|StructField { name, ty, .. }| writeln!(f, "{}(z.{name});", field_ty(ty)))
}

pub fn generate(f: &mut impl Write, type_def: &TypeDef) -> Result {
    gen_input_encoders(f, type_def)?;

    let mut output_tys = InterfacePath::default();
    output_tys.add_tys(type_def.funcs.iter().map(|func| &func.retn), &type_def.ctx);

    let mut unions = vec![];

    writeln!(f, "const struct = {{")?;

    for path in output_tys.paths {
        let ident = to_camel_case(path, ':');
        writeln!(f, "{ident}(d: use.Decoder) {{")?;

        match &type_def.ctx.costom_types[path] {
            CustomTypeKind::Unit(data) => {
                unions.push(path);

                let items = fmt!(|f| {
                    data.fields
                        .iter()
                        .enumerate()
                        .try_for_each(|(i, UnitField { name, .. })| {
                            writeln!(f, "case {i}: return {ident}.{name};")
                        })
                });
                write_enum(f, &ident, items)?;
            }
            CustomTypeKind::Enum(data) => {
                writeln!(f, "let x;")?;

                let items = fmt!(|f| data.fields.iter().enumerate().try_for_each(
                    |(i, EnumField { name, kind, .. })| {
                        writeln!(f, "case {i}: x = {{\ntype: {name:?} as const,")?;
                        match kind {
                            EnumKind::Struct(fields) => write_decoder_struct(f, fields)?,
                            EnumKind::Tuple(fields) => {
                                for (i, TupleField { doc, ty }) in fields.iter().enumerate() {
                                    write_doc_comments(f, doc)?;
                                    writeln!(f, " {i}: {}(),", field_ty(&ty))?;
                                }
                            }
                            EnumKind::Unit => {}
                        }
                        writeln!(f, "}};\nreturn x as typeof x;")
                    },
                ));
                write_enum(f, &ident, items)?;
            }
            CustomTypeKind::Struct(data) => {
                writeln!(f, "return {{")?;
                write_decoder_struct(f, &data.fields)?;
                writeln!(f, "}}")?;
            }
            CustomTypeKind::Tuple(data) => {
                let tys = data.fields.iter().map(|f| field_ty(&f.ty));
                writeln!(f, "return d.tuple({})();", join(tys, ", "))?;
            }
        }
        writeln!(f, "}},")?;
    }
    writeln!(f, "}}")?;

    // for path in unions {
    //     let ident = to_camel_case(path, ':');
    //     gen_type(f, ident, &type_def.ctx.costom_types[path])?;
    // }

    Ok(())
}

fn write_decoder_struct(f: &mut impl Write, fields: &Vec<StructField>) -> Result {
    fields.iter().try_for_each(|StructField { doc, name, ty }| {
        write_doc_comments(f, doc)?;
        writeln!(f, "{name}: {}(),", field_ty(ty))
    })
}

fn write_tuple(f: &mut impl Write, tys: impl Iterator<Item = Ty>)  {
    // let tys = data.fields.iter().map(|f| field_ty(&f.ty));
    // writeln!(f, "return d.tuple({})();", join(tys, ", "))?;
    // Ok(())
}

fn field_ty(ty: &Ty) -> String {
    match ty {
        Ty::u8 => "d.u8",
        Ty::u16 => "d.u16",
        Ty::u32 => "d.u32",
        Ty::u64 => "d.u64",
        Ty::i8 => "d.i8",
        Ty::i16 => "d.i16",
        Ty::i32 => "d.i32",
        Ty::i64 => "d.i64",
        Ty::f32 => "d.f32",
        Ty::f64 => "d.f64",
        Ty::i128 | Ty::u128 => unimplemented!(),

        Ty::bool => "d.bool",

        Ty::char => "d.char",
        Ty::String => "d.str",

        Ty::Option(ty) => return format!("d.option({})", field_ty(ty)),
        Ty::Result(ty) => return format!("d.result({}, {})", field_ty(&ty.0), field_ty(&ty.1)),

        Ty::Tuple(ty) => {
            return format!("d.tuple({})", join(ty.iter().map(field_ty), ", "));
        }
        Ty::Array { len, ty } => {
            return match **ty {
                Ty::u8 => format!("d.u8_arr({len})"),
                Ty::u16 => format!("d.u16_arr({len})"),
                Ty::u32 => format!("d.u32_arr({len})"),
                Ty::u64 => format!("d.u64_arr({len})"),
                Ty::i8 => format!("d.i8_arr({len})"),
                Ty::i16 => format!("d.i16_arr({len})"),
                Ty::i32 => format!("d.i32_arr({len})"),
                Ty::i64 => format!("d.i64_arr({len})"),
                Ty::f32 => format!("d.f32_arr({len})"),
                Ty::f64 => format!("d.f64_arr({len})"),
                ref ty => format!("d.arr({}, {len})", field_ty(ty)),
            }
        }
        Ty::Set { ty, .. } => return format!("d.set({})", field_ty(ty)),
        Ty::Map { ty, .. } => return format!("d.map({}, {})", field_ty(&ty.0), field_ty(&ty.1)),
        Ty::CustomType(path) => {
            let ident = to_camel_case(path, ':');
            return format!("this.{ident}.bind(this, d)");
        }
    }
    .to_string()
}

fn write_enum(f: &mut impl Write, ident: &String, items: fmt!(type)) -> Result {
    writeln!(f, "const num = d.len_u15();")?;
    writeln!(f, "switch (num) {{")?;
    writeln!(f, "{items:?}")?;
    writeln!(
        f,
        "default: throw new Error('Unknown discriminant of `{ident}`: ' + num)"
    )?;
    writeln!(f, "}}")
}

#[test]
#[rustfmt::skip]
fn test_field_ty() {
    use Ty::*;
    let ty = Tuple(vec![
        Option(Box::new(bool)),
        Result(Box::new((CustomType("::path::ident".into()), String))),
        Map {
            variant: MapVariant::BTreeMap,
            ty: Box::new((
                String,
                Set { variant: SetVariant::BTreeSet, ty: Box::new(u8), },
            )),
        },
    ]);

    assert_eq!(
        field_ty(&ty),
        "d.tuple(d.option(d.bool), d.result(this.PathIdent.bind(this, d), d.str), d.map(d.str, d.set(d.u8)))"
    );
}
