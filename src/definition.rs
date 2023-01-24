use std::future::Future;
use frpc_message::{Context, Message, Ty};

pub fn async_fn_ty<Func, Args, Ret>(_: &Func, ctx: &mut Context) -> (Vec<Ty>, Ty)
where
    Func: crate::fn_once::FnOnce<Args>,
    Func::Output: Future<Output = Ret>,
    Args: Message,
    Ret: Message,
{
    let Ty::Tuple(types) = Args::ty(ctx) else { unreachable!() };
    (types, Ret::ty(ctx))
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
    pub ctx: Context,
    pub funcs: Vec<Func>,
}
