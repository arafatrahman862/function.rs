use crate::Provider;

pub mod decoder;
pub mod encoder;
pub mod stub;

use crate::{
    fmt,
    utils::{to_camel_case, write_doc_comments},
};
use frpc_message::*;
use std::fmt::{Result, Write};

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
