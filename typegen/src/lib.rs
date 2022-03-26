mod ty;
mod typescript;

use std::fmt::{Debug, Formatter, Result};

// -------------------------------------------------------------------------------

#[derive(Clone, PartialEq)]
pub struct Enum {
    name: String,
    entries: Vec<(String, String)>,
}

impl Enum {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            entries: vec![],
        }
    }
    pub fn entry(&mut self, name: impl Into<String>, value: impl ToString) -> &mut Self {
        self.entries.push((name.into(), value.to_string()));
        self
    }
}

// -----------------------------------------------------------------------------

#[derive(Clone, PartialEq)]
pub struct Field {
    name: String,
    ty: Type,
}

#[derive(Clone, PartialEq)]
pub struct Struct {
    name: String,
    fields: Vec<Field>,
}

impl Struct {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            fields: vec![],
        }
    }
    pub fn field(&mut self, name: impl Into<String>, ty: Type) -> &mut Self {
        self.fields.push(Field {
            name: name.into(),
            ty,
        });
        self
    }
}

// -----------------------------------------------------------------------------

#[derive(Clone, PartialEq)]
pub struct Func {
    name: String,
    args: Vec<Field>,
    ret: Type,
}

impl Func {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            args: vec![],
            ret: Type::Tuple(Vec::new()),
        }
    }
    pub fn arg(&mut self, name: impl Into<String>, ty: Type) -> &mut Self {
        self.args.push(Field {
            name: name.into(),
            ty,
        });
        self
    }
    pub fn ret(&mut self, ty: Type) -> &mut Self {
        self.ret = ty;
        self
    }
}

// -----------------------------------------------------------------------------

#[derive(Clone, PartialEq)]
pub enum Variant {
    Unit(String),
    Tuple(String, Vec<Type>),
    Named(String, Vec<Field>),
}

#[derive(Clone, PartialEq)]
pub struct Union {
    name: String,
    /// It doesn't support Enum though. Use `Type::Enum` instade
    variants: Vec<Variant>,
}

impl Union {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            variants: vec![],
        }
    }
    pub fn variant(&mut self, variant: Variant) -> &mut Self {
        self.variants.push(variant);
        self
    }
}

// -----------------------------------------------------------------------------

#[non_exhaustive]
#[derive(Clone, PartialEq)]
pub enum Type {
    Func(Box<Func>),
    Struct(Struct),
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
    Array(Box<Type>, usize),

    Any,
    Tuple(Vec<Type>),

    Option(Box<Type>),
    Result(Box<(Type, Type)>),
}

impl Type {
    pub fn arr(self, len: usize) -> Self {
        Type::Array(Box::new(self), len)
    }
    pub fn list(self) -> Self {
        Type::Vec(Box::new(self))
    }
    pub fn optional(self) -> Self {
        Type::Option(Box::new(self))
    }
    pub fn result(ok: Type, err: Type) -> Self {
        Self::Result(Box::new((ok, err)))
    }
}
