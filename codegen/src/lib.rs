#![allow(warnings)]

use std::future::Future;
pub use typegen::{GetType, Type};

pub const fn async_fn_ty<Func, Args, Ret>(_: &Func) -> (&[Type],  Type)
where
    Func: std_trait::FnOnce<Args>,
    Func::Output: Future<Output = Ret>,
    Args: GetType,
    Ret: GetType,
{
    (
        match Args::TYPE {
            Type::Tuple(types) => types,
            _ => unreachable!(),
        },
        Ret::TYPE,
    )
}

#[derive(Debug, Clone)]
pub struct Func {
    pub name: &'static str,
    pub args: &'static [Type],
    pub retn: Type,
}

#[derive(Debug, Clone, Default)]
pub struct TypeDef {
    pub name: &'static str,
    pub version: &'static str,
    pub funcs: &'static [Func],
}