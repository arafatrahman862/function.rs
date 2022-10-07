#![doc = include_str!("../README.md")]

mod basic;
mod collection;
mod encoder;
mod wrapper;

pub use collection::{MapVariant, SetVariant};
use std::any::{type_name, TypeId};

pub trait GetType {
    fn get_type() -> Type;
}

#[non_exhaustive]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Type {
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

    bool,
    char,

    /// String slice (`&str`)
    str,
    String,

    Option(Box<Type>),
    Result(Box<(Type, Type)>),

    Slice(Box<Type>),
    Tuple(Box<[Type]>),
    TupleStruct {
        type_id: TypeId,
        name: String,
        fields: Box<[Type]>,
    },
    Struct {
        type_id: TypeId,
        name: String,
        fields: Box<[(String, Type)]>,
    },
    Enum {
        type_id: TypeId,
        name: String,
        fields: Box<[(String, Type)]>,
    },
    Array {
        len: usize,
        ty: Box<Type>,
    },
    Set {
        variant: SetVariant,
        ty: Box<Type>,
    },
    Map {
        variant: MapVariant,
        ty: Box<(Type, Type)>,
    },
}

impl Type {
    pub fn ty_id(&self) -> u8 {
        match self {
            Type::u8 => 0,
            Type::u16 => 1,
            Type::u32 => 2,
            Type::u64 => 3,
            Type::u128 => 4,
            Type::usize => 5,
            Type::i8 => 6,
            Type::i16 => 7,
            Type::i32 => 8,
            Type::i64 => 9,
            Type::i128 => 10,
            Type::isize => 11,
            Type::f32 => 12,
            Type::f64 => 13,
            Type::bool => 14,
            Type::char => 15,
            Type::str => 16,
            Type::String => 17,
            Type::Option(_) => 18,
            Type::Result(_) => 19,
            Type::Slice(_) => 20,
            Type::Tuple(_) => 21,
            Type::TupleStruct { .. } => 22,
            Type::Struct { .. } => 23,
            Type::Enum { .. } => 24,
            Type::Array { .. } => 25,
            Type::Set { .. } => 26,
            Type::Map { .. } => 27,
        }
    }
}
