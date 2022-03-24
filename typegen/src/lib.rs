#![allow(warnings)]
mod typescript;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Field {
    name: String,
    ty: Type,
}

#[derive(Debug, Clone)]
pub enum Variant {
    Unit(String),
    Tuple(String, Vec<Type>),
    Named(String, Vec<Field>),
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Type {
    Struct {
        name: String,
        fields: Vec<Field>,
    },
    Fn {
        name: String,
        args: Vec<Field>,
        ret: Box<Type>,
    },
    Union {
        name: String,
        /// Rust supports algebraic data types, Support (Unit, Tuple, Named)
        /// It doesn't support Enum though. Use `Type::Enum` instade
        variants: Vec<Variant>,
    },
    Enum {
        /// Name of the enum, and numeric type (i8, u32 ..)
        field: Box<Field>,
        /// Key, Value pairs
        entries: Vec<(String, isize)>,
    },

    U8,
    U16,
    U32,
    U64,
    U128,

    I8,
    I16,
    I32,
    I64,
    I128,

    F32,
    F64,

    Bool,

    String,
    Vec(Box<Type>),

    Any,
    Null,

    Option(Box<Type>),
    Result(Box<Type>, Box<Type>),
}