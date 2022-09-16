mod basic;
mod collections;
mod fn_ty;
mod wrapper;

pub use fn_ty::FnType;

pub trait GetType {
    fn get_ty() -> Type;
}

#[derive(Debug)]
pub enum SetType {
    BTreeSet,
    HashSet,
    BinaryHeap,
    LinkedList,
    VecDeque,
    Vec,
}

#[derive(Debug)]
pub enum MapType {
    HashMap,
    BTreeMap,
}

#[non_exhaustive]
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Type {
    char,
    bool,

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

    /// String slice (`&str`)
    str,
    String,

    Set {
        collection_ty: SetType,
        ty: Box<Type>,
    },
    Map {
        collection_ty: MapType,
        ty: Box<(Type, Type)>,
    },

    Slice(Box<Type>),
    Tuple(Box<[Type]>),
    TupleStruct {
        name: String,
        fields: Box<[Type]>,
    },
    Struct {
        name: String,
        fields: Box<[(String, Type)]>,
    },
    Enum {
        name: String,
        varients: Box<[(String, Type)]>,
    },
    Array {
        len: usize,
        ty: Box<Type>,
    },
    Option(Box<Type>),
    Result(Box<(Type, Type)>),
    Fn {
        name: String,
        args: Box<[Type]>,
        ret_ty: Box<Type>,
    },
}