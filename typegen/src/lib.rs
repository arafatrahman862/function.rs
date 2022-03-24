#![allow(warnings)]
use std::fmt::{Debug, Formatter, Result};
mod typescript;

#[derive(Clone)]
pub struct Enum {
    name: String,
    entries: Vec<(String, String)>,
}

// -----------------------------------------------------------------------------

#[derive(Clone)]
pub struct Field {
    name: String,
    ty: Type,
}

#[derive(Clone)]
pub struct Struct {
    name: String,
    fields: Vec<Field>,
}

// -----------------------------------------------------------------------------

#[derive(Clone)]
pub struct Func {
    name: String,
    args: Vec<Field>,
    ret: Type,
}

// -----------------------------------------------------------------------------

#[derive(Clone)]
pub enum Variant {
    Unit(String),
    Tuple(String, Vec<Type>),
    Named(String, Vec<Field>),
}

#[derive(Clone)]
pub struct Union {
    name: String,
    /// Rust supports algebraic data types, Support (Unit, Tuple, Named)
    /// It doesn't support Enum though. Use `Type::Enum` instade
    variants: Vec<Variant>,
}

// -----------------------------------------------------------------------------

#[non_exhaustive]
#[derive(Clone)]
pub enum Type {
    Struct(Struct),
    Func(Box<Func>),
    Union(Union),
    Enum(Enum),

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
    Result(Box<(Type, Type)>),
}
