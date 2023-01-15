mod get_ty;

use serde::{Deserialize, Serialize};
use std::future::Future;

pub use get_ty::{GetType, Type};

pub fn async_fn_ty<Func, Args, Ret>(_: &Func) -> (Vec<Type>, Type)
where
    Func: std_trait::FnOnce<Args>,
    Func::Output: Future<Output = Ret>,
    Args: GetType,
    Ret: GetType,
{
    let Type::Tuple(types) = Args::ty() else { unreachable!() };
    (types, Ret::ty())
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Func {
    pub index: u16,
    pub name: String,
    pub args: Vec<Type>,
    pub retn: Type,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TypeDef {
    pub name: String,
    pub version: String,
    pub funcs: Vec<Func>,
}

/// ## ❌ You should not implement this trait in any type ❌
pub trait Resource: GetType + databuf::Encoder + for<'de> databuf::Decoder<'de> {}

// use codegen_macro::Resource;
// #[derive(Resource)]
struct A;

impl GetType for A {
    fn ty() -> Type {
        todo!()
    }
}
