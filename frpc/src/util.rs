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