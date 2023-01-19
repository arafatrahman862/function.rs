use std::future::Future;

use crate::message::{Message, Type};

pub fn async_fn_ty<Func, Args, Ret>(_: &Func) -> (Vec<Type>, Type)
where
    Func: crate::fn_once::FnOnce<Args>,
    Func::Output: Future<Output = Ret>,
    Args: Message,
    Ret: Message,
{
    let Type::Tuple(types) = Args::ty() else { unreachable!() };
    (types, Ret::ty())
}

#[derive(Clone, Debug)]
pub struct Func {
    pub index: u16,
    pub name: String,
    pub args: Vec<Type>,
    pub retn: Type,
}

#[derive(Clone, Debug)]
pub struct TypeDef {
    pub name: String,
    pub version: String,
    pub description: String,
    pub funcs: Vec<Func>,
}
