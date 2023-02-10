mod basic;
mod collection;
mod wrapper;

pub use collection::{MapVariant, SetVariant};

pub trait Message {
    fn ty(_: &mut Context) -> Ty;
}

#[derive(Clone, Debug)]
pub struct Func {
    pub index: u16,
    pub path: String,
    pub args: Vec<Ty>,
    pub retn: Ty,
}

#[derive(Clone, Debug)]
pub struct TypeDef {
    pub name: String,
    pub version: String,
    pub description: String,
    pub ctx: Context,
    pub funcs: Vec<Func>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Ty {
    // Never,
    u8,
    u16,
    u32,
    u64,
    u128,

    i8,
    i16,
    i32,
    i64,
    i128,

    f32,
    f64,

    bool,
    char,

    String,

    Option(Box<Ty>),
    Result(Box<(Ty, Ty)>),

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

    /// The path of the user-defined type
    ///
    /// ```
    ///    struct Bar { ... }  enum Foo { ... }
    /// //        ^^^               ^^^
    /// //           \             /
    /// //    Type::CustomType("<path>::Bar" | "<path>::Foo")
    /// ```
    CustomType(String),
}

#[derive(Default, Debug, Clone)]
pub struct Context {
    pub costom_types: std::collections::BTreeMap<String, CustomTypeKind>,
}

#[derive(Debug, Clone)]
pub enum CustomTypeKind {
    Unit(CustomType<UnitField>),
    Enum(CustomType<EnumField>),
    Tuple(CustomType<TupleField>),
    Struct(CustomType<StructField>),
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
    pub kind: EnumKind,
}

#[derive(Debug, Clone)]
pub enum EnumKind {
    Unit,
    Struct(Vec<StructField>),
    Tuple(Vec<TupleField>),
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub doc: String,
    pub name: String,
    pub ty: Ty,
}

#[derive(Debug, Clone)]
pub struct TupleField {
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
