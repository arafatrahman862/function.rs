mod typescript;
mod utils;

use std::{
    any::type_name,
    fmt::{Debug, Formatter, Result},
    result,
};

#[derive(Clone, PartialEq)]
pub struct Enum {
    name: String,
    entries: Vec<(String, String)>,
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

// -----------------------------------------------------------------------------

#[derive(Clone, PartialEq)]
pub struct Func {
    name: String,
    args: Vec<Field>,
    ret: Type,
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

// -----------------------------------------------------------------------------

#[non_exhaustive]
#[derive(Clone, PartialEq)]
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
    Tuple(Vec<Type>),

    Option(Box<Type>),
    Result(Box<(Type, Type)>),
}

impl Type {
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

impl Type {
    pub fn from<T: ?Sized>(_: &T) -> result::Result<Type, String> {
        Self::from_ty::<T>()
    }
    pub fn from_ty<T: ?Sized>() -> result::Result<Self, String> {
        Self::from_str(type_name::<T>())
    }
    pub fn from_str(string: &str) -> result::Result<Self, String> {
        let ty = match string {
            "u8" => Type::U8,
            "u16" => Type::U16,
            "u32" => Type::U32,
            "u64" => Type::U64,
            "u128" => Type::U128,
            "i8" => Type::I8,
            "i16" => Type::I16,
            "i32" => Type::I32,
            "i64" => Type::I64,
            "i128" => Type::I128,
            "f32" => Type::F32,
            "f64" => Type::F64,
            "bool" => Type::Bool,
            "&str" => Type::String,
            "alloc::string::String" => Type::String,
            "()" => Type::Tuple(Vec::new()),

            ty if ty.starts_with("(") => Type::Tuple(
                utils::split_items_outside_group(utils::parse_angle_bracket_inner(ty, '(', ')'))
                    .iter()
                    .map(|item| Type::from_str(item.trim()))
                    .collect::<result::Result<Vec<_>, _>>()?,
            ),
            ty if ty.starts_with("alloc::vec::Vec") => {
                let ty_str = utils::parse_angle_bracket_inner(ty, '<', '>');
                Type::Vec(Box::new(Type::from_str(&ty_str)?))
            }
            ty if ty.starts_with("core::option::Option") => {
                let ty_str = utils::parse_angle_bracket_inner(ty, '<', '>');
                Type::Option(Box::new(Type::from_str(&ty_str)?))
            }
            ty if ty.starts_with("core::result::Result") => {
                let ty_str = utils::parse_angle_bracket_inner(ty, '<', '>');
                let result_ty = utils::split_items_outside_group(ty_str);
                Type::Result(Box::new((
                    Type::from_str(&result_ty[0])?,
                    Type::from_str(&result_ty[1])?,
                )))
            }
            ty => return Err(format!("Unknown type: `{ty}`")),
        };
        Ok(ty)
    }
}

#[test]
#[cfg(test)]
fn from_ty_name() {
    assert_eq!(Type::from_ty::<u8>(), Ok(Type::U8));
    assert_eq!(Type::from_ty::<u16>(), Ok(Type::U16));
    assert_eq!(Type::from_ty::<u32>(), Ok(Type::U32));
    assert_eq!(Type::from_ty::<u64>(), Ok(Type::U64));
    assert_eq!(Type::from_ty::<u128>(), Ok(Type::U128));
    assert_eq!(Type::from_ty::<i8>(), Ok(Type::I8));
    assert_eq!(Type::from_ty::<i16>(), Ok(Type::I16));
    assert_eq!(Type::from_ty::<i32>(), Ok(Type::I32));
    assert_eq!(Type::from_ty::<i64>(), Ok(Type::I64));
    assert_eq!(Type::from_ty::<i128>(), Ok(Type::I128));
    assert_eq!(Type::from_ty::<f32>(), Ok(Type::F32));
    assert_eq!(Type::from_ty::<f64>(), Ok(Type::F64));
    assert_eq!(Type::from_ty::<bool>(), Ok(Type::Bool));
    assert_eq!(Type::from_ty::<String>(), Ok(Type::String));
    assert_eq!(Type::from_ty::<()>(), Ok(Type::Tuple(Vec::new())));
    assert_eq!(Type::from_ty::<Option<u8>>(), Ok(Type::U8.optional()));
    assert_eq!(
        Type::from_ty::<(u8, u16)>(),
        Ok(Type::Tuple(vec![Type::U8, Type::U16]))
    );
    assert_eq!(
        Type::from_ty::<Vec<String>>(),
        Ok(Type::Vec(Box::new(Type::String)))
    );
    assert_eq!(
        Type::from_ty::<result::Result<u8, u16>>(),
        Ok(Type::result(Type::U8, Type::U16))
    );
}
