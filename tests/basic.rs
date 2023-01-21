#![allow(warnings)]
use frpc::{Message, message::Message};
use std::collections::HashMap;

#[allow(non_camel_case_types)]
#[derive(Debug, Message, Clone, PartialEq)]
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
        ty: Box<Type>,
    },
    Map {
        ty: Box<(Type, Type)>,
    },

    Enum(String),
    Union(String),
    Struct(String),
    TupleStruct(String),
}

#[derive(Debug, Message, Clone)]
pub struct EnumField {
    pub doc: String,
    pub name: String,
    pub value: isize,
}

#[derive(Debug, Message, Clone)]
pub struct UnionField {
    pub doc: String,
    pub name: String,
    pub kind: UnionKind,
}

#[derive(Debug, Message, Clone)]
pub enum UnionKind {
    Unit,
    Struct(StructField),
    Tuple(TupleStructField),
}

#[derive(Debug, Message, Clone)]
pub struct StructField {
    pub doc: String,
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Message, Clone)]
pub struct TupleStructField {
    pub doc: String,
    pub ty: Type,
}


#[derive(Debug, Message, Clone)]
pub struct CostomType<Field> {
    pub doc: String,
    pub fields: Vec<Field>,
}

#[derive(Default, Debug, Message, Clone)]
pub struct Definition {
    pub enums: HashMap<String, CostomType<EnumField>>,
    pub unions: HashMap<String, CostomType<UnionField>>,
    pub structs: HashMap<String, CostomType<StructField>>,
    pub tuple_structs: HashMap<String, CostomType<TupleStructField>>,
}

#[test]
fn test_name() {
    let mut d = frpc::message::Definition::default();
    // HashMap::<String, CostomType<UnionField>>::ty(&mut d);
    // Definition::ty(&mut d);
    CostomType::<u8>::ty(&mut d);
    std::fs::write("log.txt", format!("{d:#?}")).unwrap();
}
