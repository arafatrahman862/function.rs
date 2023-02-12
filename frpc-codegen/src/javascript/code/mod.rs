mod interface_path;

use super::interface::{fmt_js_ty, gen_type};
use crate::{
    fmt,
    utils::{to_camel_case, write_doc_comments},
};
use frpc_message::*;
use interface_path::InterfacePath;
use std::fmt::{Result, Write};

pub fn generate<'def>(type_def: &'def TypeDef) -> fmt!(type 'def) {
    fmt!(|f| {
        let mut input_interface = InterfacePath::new(&type_def.ctx);
        let mut output_interface = InterfacePath::new(&type_def.ctx);

        input_interface.add_tys(type_def.funcs.iter().flat_map(|func| func.args.iter()));
        output_interface.add_tys(type_def.funcs.iter().map(|func| &func.retn));

        for path in type_def.ctx.costom_types.keys() {
            let ident = to_camel_case(path, ':');
            gen_type(f, ident, &type_def.ctx.costom_types[path])?;
        }

        write_decoders(f, output_interface.paths, type_def)?;
        write_encoders(f, input_interface.paths, type_def)?;
        write_rpc(f, type_def)
    })
}

fn write_rpc(f: &mut impl Write, type_def: &TypeDef) -> Result {
    writeln!(f, "export default class mod {{")?;
    writeln!(f, "constructor(private rpc: use.RPC) {{}}")?;
    writeln!(f, "static close(this: mod) {{ this.rpc.close() }}")?;

    type_def.funcs.iter().try_for_each(
        |Func {
             index,
             path,
             args,
             retn,
         }| {
            let ident = path.replace("::", "_");

            write!(f, "{ident}(")?;
            for (num, ty) in args.iter().enumerate() {
                write!(f, "_{num}: {}, ", fmt_js_ty(ty))?;
            }
            writeln!(f, ") {{")?;

            writeln!(f, "const fn = this.rpc.unary_call()")?;
            writeln!(f, "const d = new use.BufWriter(fn);")?;
            writeln!(f, "d.u16({index});")?;

            for (num, arg) in args.iter().enumerate() {
                match arg {
                    Ty::CustomType(path) => {
                        writeln!(f, "extern.{}(d, _{num});", to_camel_case(path, ':'))?
                    }
                    ty => writeln!(f, "{}(_{num});", fmt_ty(ty, "extern"))?,
                };
            }
            writeln!(f, "d.flush();")?;

            if !retn.is_empty_tuple() {
                writeln!(
                    f,
                    "return fn.output().then(buf => new use.Decoder(new Uint8Array(buf)))"
                )?;
                let res = match retn {
                    Ty::CustomType(path) => format!("struct.{}", to_camel_case(path, ':')),
                    ty => format!("d => {}()", fmt_ty(ty, "struct")),
                };
                writeln!(f, ".then({res});")?;
            }
            writeln!(f, "}}")
        },
    )?;
    writeln!(f, "}}")
}

fn write_decoders<'a>(f: &mut impl Write, paths: Vec<&'a String>, type_def: &'a TypeDef) -> Result {
    writeln!(f, "let struct = {{")?;
    for path in paths {
        let ident = to_camel_case(path, ':');
        writeln!(f, "{ident}(d: use.Decoder): {ident} {{")?;

        match &type_def.ctx.costom_types[path] {
            CustomTypeKind::Unit(data) => {
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
                let items = fmt!(|f| data.fields.iter().enumerate().try_for_each(
                    |(i, EnumField { name, kind, .. })| {
                        writeln!(f, "case {i}: return {{\ntype: {name:?},")?;
                        match kind {
                            EnumKind::Struct(fields) => write_decoder_struct(f, fields)?,
                            EnumKind::Tuple(fields) => {
                                for (i, TupleField { doc, ty }) in fields.iter().enumerate() {
                                    write_doc_comments(f, doc)?;
                                    writeln!(f, " {i}: {}(),", fmt_ty(&ty, "struct"))?;
                                }
                            }
                            EnumKind::Unit => {}
                        }
                        writeln!(f, "}};")
                    },
                ));
                write_enum(f, &ident, items)?;
            }
            CustomTypeKind::Struct(data) => {
                writeln!(f, "return {{")?;
                write_decoder_struct(f, &data.fields)?;
                writeln!(f, "}}")?
            }
            CustomTypeKind::Tuple(data) => {
                writeln!(f, "return {}();", fmt_tuple(&data.fields, "struct"))?;
            }
        }
        writeln!(f, "}},")?;
    }
    writeln!(f, "}}")
}

fn write_decoder_struct(f: &mut impl Write, fields: &Vec<StructField>) -> Result {
    fields.iter().try_for_each(|StructField { doc, name, ty }| {
        write_doc_comments(f, doc)?;
        writeln!(f, "{name}: {}(),", fmt_ty(ty, "struct"))
    })
}

fn write_encoders(f: &mut impl Write, paths: Vec<&String>, type_def: &TypeDef) -> Result {
    writeln!(f, "let extern = {{")?;

    for path in paths {
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
                                writeln!(f, "{}(z[{i}]);", fmt_ty(ty, "extern"))?;
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
                writeln!(f, "return {}(z);", fmt_tuple(&data.fields, "extern"))?;
            }
        }
        writeln!(f, "}},")?;
    }
    writeln!(f, "}}")
}

fn write_encoder_struct(f: &mut impl Write, fields: &Vec<StructField>) -> Result {
    fields.iter().try_for_each(|StructField { name, ty, .. }| {
        writeln!(f, "{}(z.{name});", fmt_ty(ty, "extern"))
    })
}

fn fmt_tuple<'a>(fields: &'a Vec<TupleField>, scope: &'static str) -> fmt!(type 'a) {
    fmt!(move |f| {
        write!(f, "d.tuple(")?;
        for TupleField { ty, .. } in fields.iter() {
            write!(f, "{},", fmt_ty(ty, scope))?;
        }
        write!(f, ")")
    })
}

fn fmt_ty<'a>(ty: &'a Ty, scope: &'static str) -> fmt!(type 'a) {
    fmt!(move |f| {
        match ty {
            Ty::u8 => write!(f, "d.u8"),
            Ty::u16 => write!(f, "d.u16"),
            Ty::u32 => write!(f, "d.u32"),
            Ty::u64 => write!(f, "d.u64"),
            Ty::i8 => write!(f, "d.i8"),
            Ty::i16 => write!(f, "d.i16"),
            Ty::i32 => write!(f, "d.i32"),
            Ty::i64 => write!(f, "d.i64"),
            Ty::f32 => write!(f, "d.f32"),
            Ty::f64 => write!(f, "d.f64"),
            Ty::i128 | Ty::u128 => unimplemented!(),

            Ty::bool => write!(f, "d.bool"),

            Ty::char => write!(f, "d.char"),
            Ty::String => write!(f, "d.str"),

            Ty::Option(ty) => write!(f, "d.option({})", fmt_ty(ty, scope)),
            Ty::Result(ty) => write!(
                f,
                "d.result({}, {})",
                fmt_ty(&ty.0, scope),
                fmt_ty(&ty.1, scope)
            ),

            Ty::Tuple(tys) => {
                if !tys.is_empty() {
                    write!(f, "d.tuple(")?;
                    tys.iter()
                        .try_for_each(|ty| write!(f, "{},", fmt_ty(ty, scope)))?;
                    write!(f, ")")?;
                }
                Ok(())
            }
            Ty::Array { len, ty } => match **ty {
                Ty::u8 => write!(f, "d.u8_arr({len})"),
                Ty::u16 => write!(f, "d.u16_arr({len})"),
                Ty::u32 => write!(f, "d.u32_arr({len})"),
                Ty::u64 => write!(f, "d.u64_arr({len})"),
                Ty::i8 => write!(f, "d.i8_arr({len})"),
                Ty::i16 => write!(f, "d.i16_arr({len})"),
                Ty::i32 => write!(f, "d.i32_arr({len})"),
                Ty::i64 => write!(f, "d.i64_arr({len})"),
                Ty::f32 => write!(f, "d.f32_arr({len})"),
                Ty::f64 => write!(f, "d.f64_arr({len})"),
                ref ty => write!(f, "d.arr({}, {len})", fmt_ty(ty, scope)),
            },
            Ty::Set { ty, .. } => write!(f, "d.vec({})", fmt_ty(ty, scope)),
            Ty::Map { ty, .. } => write!(
                f,
                "d.map({}, {})",
                fmt_ty(&ty.0, scope),
                fmt_ty(&ty.1, scope)
            ),
            Ty::CustomType(path) => write!(f, "{scope}.{}.bind(0, d)", to_camel_case(path, ':')),
        }
    })
}

fn write_enum(f: &mut impl Write, ident: &String, items: fmt!(type)) -> Result {
    writeln!(f, "const num = d.len_u15();\nswitch (num) {{\n{items}default: throw use.enumErr({ident:?}, num);\n}}")
}

#[test]
#[rustfmt::skip]
fn test_fmt_tuple() {
    use Ty::*;
    let tys = vec![
        Option(Box::new(bool)),
        Result(Box::new((CustomType("::path::ident".into()), String))),
        Map {
            variant: MapVariant::BTreeMap,
            ty: Box::new((
                String,
                Set { variant: SetVariant::BTreeSet, ty: Box::new(u8), },
            )),
        },
    ];
    assert_eq!(
        format!("{}", fmt_ty(&Tuple(tys), "This")),
        "d.tuple(d.option(d.bool),d.result(This.PathIdent.bind(0, d), d.str),d.map(d.str, d.vec(d.u8)),)"
    );
}
