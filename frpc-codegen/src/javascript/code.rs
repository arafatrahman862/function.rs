use crate::{
    code_formatter::write_doc_comments,
    utils::{join, to_camel_case},
};
use frpc_message::*;
use std::fmt::{Result, Write};

use super::interface::gen_type;

#[derive(Default, Debug)]
struct Interface<'a> {
    objects: Vec<&'a String>,
}

impl<'a> Interface<'a> {
    fn add_tys(&mut self, tys: impl Iterator<Item = &'a Ty>, ctx: &'a Context) {
        tys.filter_map(|ty| match ty {
            Ty::CustomType(path) => Some(path),
            _ => None,
        })
        .for_each(|path| self.add(path, ctx));
    }

    fn add(&mut self, path: &'a String, ctx: &'a Context) {
        self.objects.push(path);
        match &ctx.costom_types[path] {
            CustomTypeKind::Enum(data) => {
                for data in data.fields.iter() {
                    match &data.kind {
                        EnumKind::Tuple(fields) => self.add_tys(fields.iter().map(|f| &f.ty), ctx),
                        EnumKind::Struct(fields) => self.add_tys(fields.iter().map(|f| &f.ty), ctx),
                        EnumKind::Unit => {}
                    }
                }
            }
            CustomTypeKind::Tuple(data) => {
                self.add_tys(data.fields.iter().map(|f| &f.ty), ctx);
            }
            CustomTypeKind::Struct(data) => {
                self.add_tys(data.fields.iter().map(|f| &f.ty), ctx);
            }
            CustomTypeKind::Unit(_) => {}
        }
    }
}

fn gen_input_encoders(f: &mut impl Write, type_def: &TypeDef) -> Result {
    let mut input_tys = Interface::default();
    input_tys.add_tys(
        type_def.funcs.iter().flat_map(|func| func.args.iter()),
        &type_def.ctx,
    );

    writeln!(f, "const extern = {{")?;

    for path in input_tys.objects {
        let ident = to_camel_case(path, ':');
        match &type_def.ctx.costom_types[path] {
            CustomTypeKind::Unit(_data) => {}
            CustomTypeKind::Enum(_data) => {}
            CustomTypeKind::Tuple(_data) => {}
            CustomTypeKind::Struct(data) => {
                write!(f, "{ident}(d: use.BufWriter, z: any) {{")?;
                for StructField { name, ty, .. } in &data.fields {
                    writeln!(f, "{}(z.{name});", field_ty(ty))?;
                }
                write!(f, "}},")?;
            }
        }
    }

    writeln!(f, "}}")?;
    Ok(())
}

pub fn generate(f: &mut impl Write, type_def: &TypeDef) -> Result {
    gen_input_encoders(f, type_def)?;

    let mut output_tys = Interface::default();
    output_tys.add_tys(type_def.funcs.iter().map(|func| &func.retn), &type_def.ctx);

    let mut unions = vec![];

    writeln!(f, "const struct = {{")?;

    for path in output_tys.objects {
        let ident = to_camel_case(path, ':');
        writeln!(f, "{ident}(d: use.Decoder) {{")?;

        match &type_def.ctx.costom_types[path] {
            CustomTypeKind::Unit(data) => {
                let items = data.fields.iter().map(|f| format!("{ident}.{}", f.name));
                unions.push(path);
                write_enum(f, &ident, items)?;
            }
            CustomTypeKind::Enum(data) => {
                let items = data
                    .fields
                    .iter()
                    .map(|EnumField { name, kind, .. }| match kind {
                        EnumKind::Unit => format!("{{ type: {name:?} }}"),
                        EnumKind::Struct(fields) => {
                            let mut s = String::from("\n");
                            write_struct_fields(&mut s, fields).unwrap();
                            format!("{{ type: {name:?} as const, {s}}}")
                        }
                        EnumKind::Tuple(fields) => {
                            let strings = fields
                                .iter()
                                .enumerate()
                                .map(|(i, f)| format!(" {i}: {}()", field_ty(&f.ty)));

                            format!("{{ type: {name:?} as const,{} }}", join(strings, ","))
                        }
                    });

                write_enum(f, &ident, items)?;
            }
            CustomTypeKind::Struct(data) => {
                writeln!(f, "return {{")?;
                write_struct_fields(f, &data.fields)?;
                writeln!(f, "}}")?;
            }
            CustomTypeKind::Tuple(data) => {
                let tys = data.fields.iter().map(|f| field_ty(&f.ty));
                writeln!(f, "return d.tuple({});", join(tys, ", "))?;
            }
        }
        writeln!(f, "}},")?;
    }
    writeln!(f, "}}")?;

    for path in unions {
        let ident = to_camel_case(path, ':');
        gen_type(f, ident, &type_def.ctx.costom_types[path])?;
    }

    Ok(())
}

fn write_struct_fields(f: &mut impl Write, fields: &Vec<StructField>) -> Result {
    for StructField { doc, name, ty } in fields.iter() {
        write_doc_comments(f, doc)?;
        writeln!(f, "{name}: {}(),", field_ty(ty))?;
    }
    Ok(())
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

fn write_enum<I>(f: &mut impl Write, ident: &String, items: I) -> Result
where
    I: Iterator<Item = String>,
{
    writeln!(f, "const num = d.len_u15();")?;
    writeln!(f, "switch (num) {{")?;
    for (i, item) in items.enumerate() {
        writeln!(f, "case {i}: return {item};")?;
    }
    writeln!(
        f,
        "default: throw new Error('Unknown discriminant of `{ident}`: ' + num)"
    )?;
    f.write_str("}")
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
        "d.tuple(d.option(d.bool), d.result(PathIdent, d.str), d.map(d.str, d.set(d.u8)))"
    );
}
