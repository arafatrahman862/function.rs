use std::future::Future;

use crate::message::{Context, Message, Ty};

pub fn async_fn_ty<Func, Args, Ret>(_: &Func, def: &mut Context) -> (Vec<Ty>, Ty)
where
    Func: crate::fn_once::FnOnce<Args>,
    Func::Output: Future<Output = Ret>,
    Args: Message,
    Ret: Message,
{
    let Ty::Tuple(types) = Args::ty(def) else { unreachable!() };
    (types, Ret::ty(def))
}

#[derive(Clone, Debug)]
pub struct Func {
    pub index: u16,
    pub name: String,
    pub args: Vec<Ty>,
    pub retn: Ty,
}

#[derive(Clone, Debug)]
pub struct TypeDef {
    pub name: String,
    pub version: String,
    pub description: String,
    pub definition: Context,
    pub funcs: Vec<Func>,
}
