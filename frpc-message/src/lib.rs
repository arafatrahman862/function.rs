mod basic;
mod collection;
mod wrapper;

#[cfg(feature = "codegen")]
use databuf::Decoder;
#[cfg(not(feature = "codegen"))]
use databuf::Encoder;

pub use collection::{MapVariant, SetVariant};

pub trait Message {
    fn ty(_: &mut Context) -> Ty;
}

// #[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "codegen", derive(Decoder))]
#[cfg_attr(not(feature = "codegen"), derive(Encoder))]
pub struct Func {
    pub index: u16,
    pub path: String,
    pub args: Vec<Ty>,
    pub retn: Ty,
}

// #[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "codegen", derive(Decoder))]
#[cfg_attr(not(feature = "codegen"), derive(Encoder))]
pub struct TypeDef {
    pub name: String,
    pub version: String,
    pub description: String,
    pub ctx: Context,
    pub funcs: Vec<Func>,
}

impl TypeDef {
    #[cfg(feature = "codegen")]
    fn from_bytes(buf: impl AsRef<[u8]>) -> Self {
        databuf::Decoder::decode(buf.as_ref()).unwrap()
    }
    
    #[cfg(not(feature = "codegen"))]
    pub fn to_bytes(&self) -> Vec<u8> {
        TypeDef::encode(&self)
    }
}

#[allow(non_camel_case_types)]
// #[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "codegen", derive(Decoder))]
#[cfg_attr(not(feature = "codegen"), derive(Encoder))]
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

impl Ty {
    pub fn is_empty_tuple(&self) -> bool {
        match self {
            Ty::Tuple(tys) => tys.is_empty(),
            _ => false,
        }
    }
}

// #[derive(Default, Debug, Clone, Hash)]
#[derive(Default)]
#[cfg_attr(feature = "codegen", derive(Decoder))]
#[cfg_attr(not(feature = "codegen"), derive(Encoder))]
pub struct Context {
    pub costom_types: std::collections::BTreeMap<String, CustomTypeKind>,
}

// #[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "codegen", derive(Decoder))]
#[cfg_attr(not(feature = "codegen"), derive(Encoder))]
pub enum CustomTypeKind {
    Unit(CustomType<UnitField>),
    Enum(CustomType<EnumField>),
    Tuple(CustomType<TupleField>),
    Struct(CustomType<StructField>),
}

/// Any user defined type like: `struct`, `enum`
// #[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "codegen", derive(Decoder))]
#[cfg_attr(not(feature = "codegen"), derive(Encoder))]
pub struct CustomType<Field> {
    pub doc: String,
    pub fields: Vec<Field>,
}

// #[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "codegen", derive(Decoder))]
#[cfg_attr(not(feature = "codegen"), derive(Encoder))]
pub struct UnitField {
    pub doc: String,
    pub name: String,
    pub value: isize,
}

// #[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "codegen", derive(Decoder))]
#[cfg_attr(not(feature = "codegen"), derive(Encoder))]
pub struct EnumField {
    pub doc: String,
    pub name: String,
    pub kind: EnumKind,
}

// #[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "codegen", derive(Decoder))]
#[cfg_attr(not(feature = "codegen"), derive(Encoder))]
pub enum EnumKind {
    Unit,
    Struct(Vec<StructField>),
    Tuple(Vec<TupleField>),
}

// #[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "codegen", derive(Decoder))]
#[cfg_attr(not(feature = "codegen"), derive(Encoder))]
pub struct StructField {
    pub doc: String,
    pub name: String,
    pub ty: Ty,
}

// #[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "codegen", derive(Decoder))]
#[cfg_attr(not(feature = "codegen"), derive(Encoder))]
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
