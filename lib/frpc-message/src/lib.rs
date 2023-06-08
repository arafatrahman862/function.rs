mod basic;
mod collection;
mod wrapper;

use std::collections::btree_map;
use std::collections::BTreeMap;

#[cfg(feature = "decode")]
use databuf::Decode;
#[cfg(feature = "encode")]
use databuf::Encode;

pub use collection::{MapVariant, SetVariant};

pub trait Message {
    fn ty(_: &mut CostomTypes) -> Ty;
}

#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "encode", derive(Encode))]
pub struct Func {
    pub index: u16,
    pub path: Ident,
    pub args: Vec<Ty>,
    pub retn: Ty,
}

#[derive(Default)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "encode", derive(Encode))]
pub struct TypeDef {
    pub name: Ident,
    pub costom_types: CostomTypes,
    pub funcs: Vec<Func>,
}

impl TypeDef {
    pub fn new(name: &str, costom_types: CostomTypes, funcs: Vec<Func>) -> Self {
        Self {
            name: Ident(name.to_string()),
            costom_types,
            funcs,
        }
    }

    #[cfg(feature = "decode")]
    pub fn try_from(bytes: impl AsRef<[u8]>) -> databuf::Result<Self> {
        Self::from_bytes::<{ databuf::config::num::LEB128 }>(bytes.as_ref())
    }

    #[cfg(feature = "encode")]
    pub fn as_bytes(&self) -> Vec<u8> {
        Encode::to_bytes::<{ databuf::config::num::LEB128 }>(&self)
    }
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "encode", derive(Encode))]
pub enum Ty {
    // Never, // debug
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
        ty: Box<Ty>,
        len: usize,
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

#[derive(Default)]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "encode", derive(Encode))]
pub struct CostomTypes(BTreeMap<String, CustomTypeKind>);

impl CostomTypes {
    pub fn register(&mut self, name: String, f: fn(&mut Self) -> CustomTypeKind) -> Ty {
        if let btree_map::Entry::Vacant(entry) = self.0.entry(name.clone()) {
            entry.insert(CustomTypeKind::default());
            let costom_type_kind = f(self);
            self.0.insert(name.clone(), costom_type_kind);
        }
        Ty::CustomType(name)
    }
}

impl std::ops::Deref for CostomTypes {
    type Target = BTreeMap<String, CustomTypeKind>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for CustomTypeKind {
    fn default() -> Self {
        Self::Unit(CustomType {
            doc: "".into(),
            fields: vec![],
        })
    }
}

#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "encode", derive(Encode))]
pub enum CustomTypeKind {
    Unit(CustomType<UnitField>),
    Enum(CustomType<EnumField>),
    Tuple(CustomType<TupleField>),
    Struct(CustomType<StructField>),
}

#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "encode", derive(Encode))]
/// Any user defined type like: `struct`, `enum`
pub struct CustomType<Field> {
    pub doc: String,
    pub fields: Vec<Field>,
}

#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "encode", derive(Encode))]
pub struct UnitField {
    pub doc: String,
    pub name: Ident,
    pub value: isize,
}

#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "encode", derive(Encode))]
pub struct EnumField {
    pub doc: String,
    pub name: Ident,
    pub kind: EnumKind,
}

#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "encode", derive(Encode))]
pub enum EnumKind {
    Unit,
    Struct(Vec<StructField>),
    Tuple(Vec<TupleField>),
}

#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "encode", derive(Encode))]
pub struct StructField {
    pub doc: String,
    pub name: Ident,
    pub ty: Ty,
}

#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "encode", derive(Encode))]
pub struct TupleField {
    pub doc: String,
    pub ty: Ty,
}

// ---------------------------------------------------------------

#[derive(Default, Hash, Clone)]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "encode", derive(Encode))]
pub struct Ident(pub String);

impl std::ops::Deref for Ident {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0.trim_start_matches("r#")
    }
}

impl Ident {
    pub fn is_raw_str_literal(&self) -> bool {
        self.0.starts_with("r#")
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.trim_start_matches("r#").fmt(f)
    }
}

impl std::fmt::Debug for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.trim_start_matches("r#").fmt(f)
    }
}

// ---------------------------------------------------------------

impl<Field> CustomType<Field> {
    pub fn new(doc: &str, fields: Vec<Field>) -> Self {
        Self {
            doc: doc.to_string(),
            fields,
        }
    }
}

impl UnitField {
    pub fn new(doc: &str, name: &str, value: isize) -> Self {
        Self {
            doc: doc.to_string(),
            name: Ident(name.to_string()),
            value,
        }
    }
}

impl EnumField {
    pub fn new(doc: &str, name: &str, kind: EnumKind) -> Self {
        Self {
            doc: doc.to_string(),
            name: Ident(name.to_string()),
            kind,
        }
    }
}

impl StructField {
    pub fn new(doc: &str, name: &str, ty: Ty) -> Self {
        Self {
            doc: doc.to_string(),
            name: Ident(name.to_string()),
            ty,
        }
    }
}

impl TupleField {
    pub fn new(doc: &str, ty: Ty) -> Self {
        Self {
            doc: doc.to_string(),
            ty,
        }
    }
}
