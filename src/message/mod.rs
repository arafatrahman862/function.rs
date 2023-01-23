mod basic;
mod collection;
mod wrapper;

pub use collection::{MapVariant, SetVariant};
use std::{collections::HashMap, default};

pub trait Message {
    fn ty(_: &mut Context) -> Ty;
}

#[non_exhaustive]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Ty {
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

    Option(Box<Ty>),
    Result(Box<(Ty, Ty)>),

    Slice(Box<Ty>),
    Tuple(Vec<Ty>),

    Array {
        len: usize,
        ty: Box<Ty>,
    },
    Set {
        variant: SetVariant,
        ty: Box<Ty>,
    },
    Map {
        variant: MapVariant,
        ty: Box<(Ty, Ty)>,
    },

    /// The name of the user-defined type
    ///
    /// ```
    ///    struct Bar { ... }  enum Foo { ... }
    /// //        ^^^               ^^^
    /// //           \             /
    /// //    Type::CustomType("Bar" | "Foo")
    /// ```
    CustomType(String),
}

#[derive(Default, Debug, Clone)]
pub struct Context {
    pub costom_types: HashMap<String, CustomTypeKind>,
}

#[derive(Debug, Clone)]
pub enum CustomTypeKind {
    Unit(CustomType<UnitField>),
    Enum(CustomType<EnumField>),
    Struct(CustomType<StructField>),
    TupleStruct(CustomType<TupleStructField>),
}

/// Any user defined type like: `struct`, `enum`
#[derive(Debug, Clone)]
pub struct CustomType<Field> {
    pub doc: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct UnitField {
    pub doc: String,
    pub name: String,
    pub value: isize,
}

#[derive(Debug, Clone)]
pub struct EnumField {
    pub doc: String,
    pub name: String,
    pub kind: UnionKind,
}

#[derive(Debug, Clone)]
pub enum UnionKind {
    Unit,
    Struct(Vec<StructField>),
    Tuple(Vec<TupleStructField>),
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub doc: String,
    pub name: String,
    pub ty: Ty,
}

#[derive(Debug, Clone)]
pub struct TupleStructField {
    pub doc: String,
    pub ty: Ty,
}

//   -------------------------------------------------------------

impl Default for CustomTypeKind {
    fn default() -> Self {
        Self::Unit(CustomType {
            doc: "".into(),
            fields: vec![],
        })
    }
}

//   -------------------------------------------------------------

#[doc(hidden)]
pub mod _utils {
    pub fn s<T>(value: T) -> String
    where
        String: From<T>,
    {
        String::from(value)
    }
    pub fn c<T: Clone>(value: &T) -> T {
        Clone::clone(value)
    }
}
