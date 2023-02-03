use crate::{
    code_formatter::write_doc_comments,
    utils::{join, to_camel_case},
};
use frpc_message::*;
use std::fmt::{Result, Write};

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

pub fn generate(c: &mut impl Write, type_def: &TypeDef) -> Result {
    let mut interface = Interface::default();
    interface.add_tys(type_def.funcs.iter().map(|func| &func.retn), &type_def.ctx);

    writeln!(c, "const struct = {{")?;

    for path in interface.objects {
        let ident = to_camel_case(path, ':');
        write!(c, "{ident}: (d: use.Decoder) => ")?;

        match &type_def.ctx.costom_types[path] {
            CustomTypeKind::Unit(data) => {
                let items = data.fields.iter().map(|f| format!("{ident}.{}", f.name));

                write_enum(c, &ident, items)?;
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

                write_enum(c, &ident, items)?;
            }
            CustomTypeKind::Struct(data) => {
                writeln!(c, "({{")?;
                write_struct_fields(c, &data.fields)?;
                writeln!(c, "}}),")?;
            }
            CustomTypeKind::Tuple(data) => {
                let tys = data.fields.iter().map(|f| field_ty(&f.ty));
                writeln!(c, "d.tuple({}),", join(tys, ", "))?;
            }
        }
    }
    writeln!(c, "}}")
}

fn write_struct_fields(c: &mut impl Write, fields: &Vec<StructField>) -> Result {
    for StructField { doc, name, ty } in fields.iter() {
        write_doc_comments(c, doc)?;
        writeln!(c, "{name}: {}(),", field_ty(ty))?;
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
            return format!("struct.{ident}.bind(null, d)");
        }
    }
    .to_string()
}

fn write_enum<I>(c: &mut impl Write, ident: &String, items: I) -> Result
where
    I: Iterator<Item = String>,
{
    writeln!(c, "{{")?;

    writeln!(c, "const num = d.len_u15();")?;
    writeln!(c, "switch (num) {{")?;
    for (i, item) in items.enumerate() {
        writeln!(c, "case {i}: return {item};")?;
    }
    writeln!(
        c,
        "default: throw new Error('Unknown discriminant of `{ident}`: ' + num)"
    )?;
    c.write_str("}")?;

    writeln!(c, "}},")
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
