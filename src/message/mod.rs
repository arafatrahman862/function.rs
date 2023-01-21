mod basic;
mod collection;
mod wrapper;

pub use collection::{MapVariant, SetVariant};
use std::collections::HashMap;

pub trait Message {
    fn ty(_: &mut Definition) -> Type;
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

    Enum(String),
    Union(String),
    Struct(String),
    TupleStruct(String),
}

#[derive(Default, Debug, Clone)]
pub struct Definition {
    pub enums: HashMap<String, CostomType<EnumField>>,
    pub unions: HashMap<String, CostomType<UnionField>>,
    pub structs: HashMap<String, CostomType<StructField>>,
    pub tuple_structs: HashMap<String, CostomType<TupleStructField>>,
}

#[derive(Default, Debug, Clone)]
pub struct GenericDefinition {
    pub enums: HashMap<String, Generic<CostomType<EnumField>>>,
    pub unions: HashMap<String, Generic<CostomType<UnionField>>>,
    pub structs: HashMap<String, Generic<CostomType<StructField>>>,
    pub tuple_structs: HashMap<String, Generic<CostomType<TupleStructField>>>,
}


#[derive(Default, Debug, Clone)]
pub struct Schema {
    pub definition: Definition,
    pub generic_definition: GenericDefinition,
}

#[derive(Default, Debug, Clone)]
pub struct Generic<CostomType> {
    pub perameter: Vec<String>,
    pub costom_type: CostomType,
}

struct GenericPerameter {
    name: String,
    default: Option<Type>,
}

#[derive(Debug, Clone)]
pub struct CostomType<Field> {
    pub doc: String,
    pub fields: Vec<Field>,
}

impl<Field> CostomType<Field> {
    pub fn new() -> Self {
        Self {
            doc: Default::default(),
            fields: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnumField {
    pub doc: String,
    pub name: String,
    pub value: isize,
}

#[derive(Debug, Clone)]
pub struct UnionField {
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

// impl Type {
//     pub fn id(&self) -> u8 {
//         match self {
//             Type::Never => 0,
//             Type::u8 => 1,
//             Type::u16 => 2,
//             Type::u32 => 3,
//             Type::u64 => 4,
//             Type::u128 => 5,
//             Type::usize => 6,
//             Type::i8 => 7,
//             Type::i16 => 8,
//             Type::i32 => 9,
//             Type::i64 => 10,
//             Type::i128 => 11,
//             Type::isize => 12,
//             Type::f32 => 13,
//             Type::f64 => 14,
//             Type::bool => 15,
//             Type::char => 16,
//             Type::str => 17,
//             Type::String => 18,
//             Type::Option(_) => 19,
//             Type::Result(_) => 20,
//             Type::Slice(_) => 21,
//             Type::Tuple(_) => 22,
//             Type::TupleStruct { .. } => 23,
//             Type::Struct { .. } => 24,
//             Type::Enum { .. } => 25,
//             Type::Array { .. } => 26,
//             Type::Set { variant, .. } => match variant {
//                 SetVariant::BTreeSet => 27,
//                 SetVariant::HashSet => 28,
//                 SetVariant::BinaryHeap => 29,
//                 SetVariant::LinkedList => 30,
//                 SetVariant::VecDeque => 31,
//                 SetVariant::Vec => 32,
//             },
//             Type::Map { variant, .. } => match variant {
//                 MapVariant::HashMap => 33,
//                 MapVariant::BTreeMap => 34,
//             },
//         }
//     }
// }
