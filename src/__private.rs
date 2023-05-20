#![doc(hidden)]
pub use frpc_message;
use frpc_message::{Context, Message, Ty};

pub fn fn_ty<Func, Args, Ret>(
    _: &Func,
    ctx: &mut Context,
    index: u16,
    path: impl Into<String>,
) -> frpc_message::Func
where
    Func: crate::fn_once::FnOnce<Args>,
    Func::Output: std::future::Future<Output = Ret>,
    Args: Message,
    Ret: Message,
{
    let Ty::Tuple(args) = Args::ty(ctx) else { unreachable!() };
    frpc_message::Func {
        index,
        path: path.into(),
        args,
        retn: Ret::ty(ctx),
    }
}

impl<T> Message for crate::State<T> {
    fn ty(_: &mut Context) -> frpc_message::Ty {
        Ty::Tuple(vec![])
    }
}
