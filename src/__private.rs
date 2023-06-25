#![doc(hidden)]
use crate::output_type::OutputType;
pub use frpc_message;
use frpc_message::{CostomTypes, Ty, TypeId};

pub fn fn_ty<Func, Args>(
    _: &Func,
    costom_types: &mut CostomTypes,
    index: u16,
    ident: &str,
) -> frpc_message::Func
where
    Func: std_lib::FnOnce<Args>,
    Args: TypeId,
    Func::Output: OutputType,
{
    let Ty::Tuple(mut args) = Args::ty(costom_types) else { unreachable!() };
    if let Some(ty) = args.first() {
        if ty.is_empty_tuple() {
            args.remove(0);
        }
    }
    frpc_message::Func {
        index,
        ident: frpc_message::Ident(ident.to_string()),
        args,
        output: <Func::Output as OutputType>::fn_output_ty(costom_types),
    }
}

impl<T> TypeId for crate::State<T> {
    fn ty(_: &mut CostomTypes) -> frpc_message::Ty {
        Ty::Tuple(vec![])
    }
}
