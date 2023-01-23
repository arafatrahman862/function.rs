mod basic;
mod collection;
mod wrapper;

pub use collection::{MapVariant, SetVariant};
use std::{collections::HashMap, default};

pub trait Message {
    fn ty(_: &mut Context) -> Type;
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

    Option(Box<Type>),
    Result(Box<(Type, Type)>),

    Slice(Box<Type>),
    Tuple(Vec<Type>),

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

    /// The name of the user-defined type
    ///
    /// ```
    ///    struct Bar { ... }  enum Foo { ... }
    /// //        ^^^               ^^^
    /// //           \             /
    /// //    Type::CustomType("Bar" | "Foo")
    /// ```
    CustomType(String),

    /// Example:
    ///
    /// ```
    ///    Record < K , V >  //          args
    /// // ^^^^^^   ^   ^
    /// //  name    |   |------>  Type::GenericPeram(1)
    /// //          |---------->  Type::GenericPeram(0)
    /// ```
    Generic {
        args: Vec<Type>,
        name: String,
    },
    GenericPeram(u8),
}

#[derive(Default, Debug, Clone)]
pub struct Context {
    pub costom_types: HashMap<String, CustomTypeKind>,
    pub generic_costom_types: HashMap<String, GenericCustomTypeKind>,
}

#[derive(Debug, Clone)]
pub enum CustomTypeKind {
    Unit(CustomType<UnitField>),
    Enum(CustomType<EnumField>),
    Struct(CustomType<StructField>),
    TupleStruct(CustomType<TupleStructField>),
}

#[derive(Debug, Clone)]
pub enum GenericCustomTypeKind {
    Enum(Generic<CustomType<EnumField>>),
    Struct(Generic<CustomType<StructField>>),
    TupleStruct(Generic<CustomType<TupleStructField>>),
}

#[derive(Debug, Clone)]
pub struct Generic<CustomType> {
    pub params: Vec<String>,
    pub costom_type: CustomType,
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
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct TupleStructField {
    pub doc: String,
    pub ty: Type,
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

impl Default for GenericCustomTypeKind {
    fn default() -> Self {
        Self::Enum(Generic {
            params: vec![],
            costom_type: CustomType {
                doc: "".into(),
                fields: vec![],
            },
        })
    }
}

//   -------------------------------------------------------------

macro_rules! generic_param {
    [$($ty:tt : $idx:literal),*] => {
        pub mod __gp {$(
            #[doc(hidden)]
            pub struct $ty; 
        )*}
        
        $(impl Message for __gp::$ty {
            fn ty(_: &mut Context) -> Type { Type::GenericPeram($idx) }
        })* 
    };
}
generic_param!(T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11, T12:12, T13:13, T14:14, T15:15);

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
