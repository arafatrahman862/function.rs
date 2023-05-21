#![doc(hidden)]
pub use frpc_message;
use frpc_message::{CostomTypes, Message, Ty};

pub fn fn_ty<Func, Args, Ret>(
    _: &Func,
    costom_types: &mut CostomTypes,
    index: u16,
    path: impl Into<String>,
) -> frpc_message::Func
where
    Func: crate::fn_once::FnOnce<Args>,
    Func::Output: std::future::Future<Output = Ret>,
    Args: Message,
    Ret: Message,
{
    let Ty::Tuple(args) = Args::ty(costom_types) else { unreachable!() };
    frpc_message::Func {
        index,
        path: path.into(),
        args,
        retn: Ret::ty(costom_types),
    }
}

impl<T> Message for crate::State<T> {
    fn ty(_: &mut CostomTypes) -> frpc_message::Ty {
        Ty::Tuple(vec![])
    }
}
