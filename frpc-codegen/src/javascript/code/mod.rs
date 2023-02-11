mod interface_path;

use super::interface::{fmt_js_ty, gen_type};
use crate::{
    fmt,
    utils::{to_camel_case, write_doc_comments},
};
use frpc_message::*;
use interface_path::InterfacePath;
use std::{
    collections::BTreeSet,
    fmt::{Debug, Result, Write},
};

pub fn generate<'def>(type_def: &'def TypeDef) -> fmt!(type 'def) {
    fmt!(|f| {
        let mut input_interface = InterfacePath::default();
        let mut output_interface = InterfacePath::default();

        output_interface.add_tys(type_def.funcs.iter().map(|func| &func.retn), &type_def.ctx);
        input_interface.add_tys(
            type_def.funcs.iter().flat_map(|func| func.args.iter()),
            &type_def.ctx,
        );

        let interface_paths: BTreeSet<_> = output_interface
            .paths
            .iter()
            .chain(input_interface.paths.iter())
            .collect();

        // ------------------------------------------------------------------------------------------

        let output_unit_interface_paths = write_decoders(f, &output_interface.paths, type_def)?;

        writeln!(f, "export namespace trait {{")?;
        for path in interface_paths {
            let ident = to_camel_case(path, ':');

            if output_unit_interface_paths.contains(&path) {
                gen_type(f, ident, &type_def.ctx.costom_types[*path])?;
            } else if output_interface.paths.contains(path) {
                writeln!(
                    f,
                    "export type {ident} = ReturnType<typeof struct.{ident}>;"
                )?;
            } else if input_interface.paths.contains(path) {
                gen_type(f, ident, &type_def.ctx.costom_types[*path])?;
            }
        }
        writeln!(f, "}}")?;

        write_encoders(f, input_interface.paths, type_def)?;
        write_rpc(f, type_def)
    })
}

fn write_rpc(f: &mut impl Write, type_def: &TypeDef) -> Result {
    writeln!(f, "export default class mod {{")?;
    writeln!(f, "constructor(private rpc: RPC) {{}}")?;
    writeln!(f, "static close(this: mod) {{ this.rpc.close() }}")?;

    type_def.funcs.iter().try_for_each(
        |Func {
             index,
             path,
             args,
             retn,
         }| {
            let ident = path.replace("::", "_");

            fn fmt_arg(ty: &Ty) -> String {
                match ty {
                    Ty::CustomType(path) => format!("trait.{}", to_camel_case(path, ':')),
                    ty => fmt_js_ty(ty),
                }
            }
            write!(f, "{ident}(")?;
            for (num, ty) in args.iter().enumerate() {
                write!(f, "_{num}: {}, ", fmt_arg(ty))?;
            }
            writeln!(f, "): {} {{", fmt_arg(retn))?;

            writeln!(f, "const fn = this.rpc.unary_call()")?;
            writeln!(f, "const d = new use.BufWriter(fn);")?;
            writeln!(f, "d.u16({index});")?;

            for (num, arg) in args.iter().enumerate() {
                match arg {
                    Ty::CustomType(path) => {
                        writeln!(f, "extern.{}(d, _{num});", to_camel_case(path, ':'))?
                    }
                    ty => writeln!(f, "{}(_{num});", fmt_ty(ty))?,
                };
            }
            writeln!(f, "d.flush();")?;
            writeln!(f, "throw new Error('todo')")?;

            // return new use.Decoder(new Uint8Array(await fn.output())).str()
            writeln!(f, "}}")
        },
    )?;
    writeln!(f, "}}")
}

fn write_decoders<'a, 'path>(
    f: &mut impl Write,
    output_interface_path: &'path Vec<&'a String>,
    type_def: &'a TypeDef,
) -> std::result::Result<Vec<&'path &'a String>, std::fmt::Error> {
    let mut output_unit_interface_paths = vec![];

    writeln!(f, "namespace struct {{")?;
    for path in output_interface_path {
        let ident = to_camel_case(path, ':');
        writeln!(f, "export function {ident}(d: use.Decoder) {{")?;

        match &type_def.ctx.costom_types[*path] {
            CustomTypeKind::Unit(data) => {
                output_unit_interface_paths.push(path);

                let items = fmt!(|f| {
                    data.fields
                        .iter()
                        .enumerate()
                        .try_for_each(|(i, UnitField { name, .. })| {
                            writeln!(f, "case {i}: return trait.{ident}.{name};")
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
                                    writeln!(f, " {i}: {}(),", fmt_ty(&ty))?;
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
                writeln!(f, "}}")?
            }
            CustomTypeKind::Tuple(data) => {
                writeln!(f, "return {}();", fmt_tuple(&data.fields))?;
            }
        }
        writeln!(f, "}}")?;
    }
    writeln!(f, "}}")?;
    Ok(output_unit_interface_paths)
}

fn write_decoder_struct(f: &mut impl Write, fields: &Vec<StructField>) -> Result {
    fields.iter().try_for_each(|StructField { doc, name, ty }| {
        write_doc_comments(f, doc)?;
        writeln!(f, "{name}: {}(),", fmt_ty(ty))
    })
}

fn write_encoders(
    f: &mut impl Write,
    input_interface_paths: Vec<&String>,
    type_def: &TypeDef,
) -> Result {
    writeln!(f, "namespace extern {{")?;

    for path in input_interface_paths {
        let ident = to_camel_case(path, ':');
        writeln!(
            f,
            "export function {ident}(d: use.BufWriter, z: trait.{ident}) {{"
        )?;

        match &type_def.ctx.costom_types[path] {
            CustomTypeKind::Unit(data) => {
                writeln!(f, "switch (z) {{")?;
                for (i, UnitField { name, .. }) in data.fields.iter().enumerate() {
                    writeln!(f, "case trait.{ident}.{name}: return d.len_u15({i});")?;
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
                                writeln!(f, "{}(z[{i}]);", fmt_ty(ty))?;
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
                writeln!(f, "return {}(z);", fmt_tuple(&data.fields))?;
            }
        }
        writeln!(f, "}}")?;
    }
    writeln!(f, "}}")
}

fn write_encoder_struct(f: &mut impl Write, fields: &Vec<StructField>) -> Result {
    fields
        .iter()
        .try_for_each(|StructField { name, ty, .. }| writeln!(f, "{}(z.{name});", fmt_ty(ty)))
}

fn fmt_tuple(fields: &Vec<TupleField>) -> fmt!(type '_) {
    fmt!(move |f| {
        write!(f, "d.tuple(")?;
        for TupleField { ty, .. } in fields.iter() {
            write!(f, "{},", fmt_ty(ty))?;
        }
        write!(f, ")")
    })
}

fn fmt_ty(ty: &Ty) -> fmt!(type '_) {
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

            Ty::Option(ty) => write!(f, "d.option({})", fmt_ty(ty)),
            Ty::Result(ty) => write!(f, "d.result({}, {})", fmt_ty(&ty.0), fmt_ty(&ty.1)),

            Ty::Tuple(tys) => {
                if !tys.is_empty() {
                    write!(f, "d.tuple(")?;
                    tys.iter().try_for_each(|ty| write!(f, "{},", fmt_ty(ty)))?;
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
                ref ty => write!(f, "d.arr({}, {len})", fmt_ty(ty)),
            },
            Ty::Set { ty, .. } => write!(f, "d.set({})", fmt_ty(ty)),
            Ty::Map { ty, .. } => write!(f, "d.map({}, {})", fmt_ty(&ty.0), fmt_ty(&ty.1)),
            Ty::CustomType(path) => write!(f, "{}.bind(0, d)", to_camel_case(path, ':')),
        }
    })
}

fn write_enum(f: &mut impl Write, ident: &String, items: fmt!(type)) -> Result {
    writeln!(f, "const num = d.len_u15();")?;
    writeln!(f, "switch (num) {{")?;
    writeln!(f, "{items}")?;
    writeln!(
        f,
        "default: throw new Error('Unknown discriminant of `{ident}`: ' + num)"
    )?;
    writeln!(f, "}}")
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
        format!("{}", fmt_ty(&Tuple(tys))),
        "d.tuple(d.option(d.bool),d.result(this.PathIdent.bind(this, d), d.str),d.map(d.str, d.set(d.u8)),)"
    );
}
