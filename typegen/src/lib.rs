#![doc = include_str!("../README.md")]

mod basic;
mod collection;
mod wrapper;

pub use collection::{MapVariant, SetVariant};

pub trait GetType {
    const TYPE: Type;
}

#[non_exhaustive]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Never,

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

    Option(&'static Type),
    Result(&'static (Type, Type)),

    Slice(&'static Type),
    Tuple(&'static [Type]),
    TupleStruct {
        name: &'static str,
        fields: &'static [Type],
    },
    Struct {
        name: &'static str,
        fields: &'static [(&'static str, Type)],
    },
    Enum {
        name: &'static str,
        fields: &'static [(&'static str, Type)],
    },
    Array {
        len: usize,
        ty: &'static Type,
    },
    Set {
        variant: SetVariant,
        ty: &'static Type,
    },
    Map {
        variant: MapVariant,
        ty: &'static (Type, Type),
    },
}

impl Type {
    pub fn idx(&self) -> u8 {
        match self {
            Type::Never => 0,
            Type::u8 => 1,
            Type::u16 => 2,
            Type::u32 => 3,
            Type::u64 => 4,
            Type::u128 => 5,
            Type::usize => 6,
            Type::i8 => 7,
            Type::i16 => 8,
            Type::i32 => 9,
            Type::i64 => 10,
            Type::i128 => 11,
            Type::isize => 12,
            Type::f32 => 13,
            Type::f64 => 14,
            Type::bool => 15,
            Type::char => 16,
            Type::str => 17,
            Type::String => 18,
            Type::Option(_) => 19,
            Type::Result(_) => 20,
            Type::Slice(_) => 21,
            Type::Tuple(_) => 22,
            Type::TupleStruct { .. } => 23,
            Type::Struct { .. } => 24,
            Type::Enum { .. } => 25,
            Type::Array { .. } => 26,
            Type::Set { variant, .. } => match variant {
                SetVariant::BTreeSet => 27,
                SetVariant::HashSet => 28,
                SetVariant::BinaryHeap => 29,
                SetVariant::LinkedList => 30,
                SetVariant::VecDeque => 31,
                SetVariant::Vec => 32,
            },
            Type::Map { variant, .. } => match variant {
                MapVariant::HashMap => 33,
                MapVariant::BTreeMap => 34,
            },
        }
    }
}

// #[test]
// fn test_name() {
//     println!("{:?}", std::mem::size_of::<&'static str>());
// }