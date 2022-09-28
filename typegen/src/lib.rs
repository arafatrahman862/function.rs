#![doc = include_str!("../README.md")]

mod basic;
mod collection;
mod fn_ty;
mod wrapper;

pub use collection::{MapVariant, SetVariant};
pub use fn_ty::AsyncFnType;
use std::any::type_name;

pub trait GetType {
    fn get_ty() -> Type;
}

#[non_exhaustive]
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Type {
    char,
    bool,

    u8,
    u16,
    u32,
    u64,
    u128,
    usize,

    i8,
    i16,
    i32,
    i64,
    i128,
    isize,

    f32,
    f64,

    /// String slice (`&str`)
    str,
    String,

    Set {
        variant: SetVariant,
        ty: Box<Type>,
    },
    Map {
        variant: MapVariant,
        ty: Box<(Type, Type)>,
    },

    Slice(Box<Type>),
    Tuple(Box<[Type]>),
    TupleStruct {
        name: String,
        fields: Box<[Type]>,
    },
    Struct {
        name: String,
        fields: Box<[(String, Type)]>,
    },
    Enum {
        name: String,
        fields: Box<[(String, Type)]>,
    },
    Array {
        len: usize,
        ty: Box<Type>,
    },
    Option(Box<Type>),
    Result(Box<(Type, Type)>),
    Fn {
        name: String,
        args: Box<[Type]>,
        ret_ty: Box<Type>,
    },
}

impl Type {
    pub fn ty_id(&self) -> u8 {
        match self {
            Type::char => 0,
            Type::bool => 1,
            Type::u8 => 2,
            Type::u16 => 3,
            Type::u32 => 4,
            Type::u64 => 5,
            Type::u128 => 6,
            Type::usize => 7,
            Type::i8 => 8,
            Type::i16 => 9,
            Type::i32 => 10,
            Type::i64 => 11,
            Type::i128 => 12,
            Type::isize => 13,
            Type::f32 => 14,
            Type::f64 => 15,
            Type::str => 16,
            Type::String => 17,
            Type::Set { .. } => 18,
            Type::Map { .. } => 19,
            Type::Slice(_) => 20,
            Type::Tuple(_) => 21,
            Type::TupleStruct { .. } => 22,
            Type::Struct { .. } => 23,
            Type::Enum { .. } => 24,
            Type::Array { .. } => 25,
            Type::Option(_) => 26,
            Type::Result(_) => 27,
            Type::Fn { .. } => 28,
        }
    }
}

// -------------------------------------------------------------------------------

use bin_layout::{Decoder, Encoder};

impl Encoder for Type {
    fn encoder(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        self.ty_id().encoder(w)?;
        match self {
            Type::Set { variant, ty } => {
                variant.ty_id().encoder(w)?;
                ty.encoder(w)
            }
            Type::Map { variant, ty } => {
                variant.ty_id().encoder(w)?;
                ty.encoder(w)
            }
            Type::Slice(ty) => ty.encoder(w),
            Type::Tuple(fields) => fields.encoder(w),
            Type::TupleStruct { name, fields } => {
                name.encoder(w)?;
                fields.encoder(w)
            }
            Type::Struct { name, fields } | Type::Enum { name, fields } => {
                name.encoder(w)?;
                fields.encoder(w)
            }
            Type::Array { len, ty } => {
                (*len as u32).encoder(w)?;
                ty.encoder(w)
            }
            Type::Option(ty) => ty.encoder(w),
            Type::Result(ty) => ty.encoder(w),
            Type::Fn { name, args, ret_ty } => {
                name.encoder(w)?;
                args.encoder(w)?;
                ret_ty.encoder(w)
            }
            _ => Ok(()),
        }
    }
}

impl Decoder<'_> for Type {
    fn decoder(r: &mut &[u8]) -> Result<Self, Box<(dyn std::error::Error + Send + Sync)>> {
        let ty_id = u8::decoder(r)?;
        Ok(match ty_id {
            0 => Type::char, 
            1 => Type::bool, 
            2 => Type::u8, 
            3 => Type::u16, 
            4 => Type::u32, 
            5 => Type::u64, 
            6 => Type::u128, 
            7 => Type::usize, 
            8 => Type::i8, 
            9 => Type::i16, 
            10 => Type::i32, 
            11 => Type::i64, 
            12 => Type::i128, 
            13 => Type::isize, 
            14 => Type::f32, 
            15 => Type::f64, 
            16 => Type::str, 
            17 => Type::String, 
            id => {
                return Err(
                    format!("Can't create `{}` from `u8`: {id}", type_name::<Self>()).into(),
                )
            }
        })
    }
}

// #[test]
// fn test_name() {
//     println!("{:?}", Type::decode(&[17]));
// }