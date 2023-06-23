use frpc_message::{CostomTypes, FuncOutput, TypeId};
use std::future::Future;

pub trait FnOutputType {
    fn fn_output_ty(_: &mut CostomTypes) -> FuncOutput;
}

impl<Fut> FnOutputType for Fut
where
    Fut: Future,
    Fut::Output: TypeId,
{
    fn fn_output_ty(c: &mut CostomTypes) -> FuncOutput {
        FuncOutput::Unary(<Fut::Output as TypeId>::ty(c))
    }
}
