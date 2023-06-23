use super::*;
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

impl<G> FnOutputType for SSE<G>
where
    G: AsyncGenerator,
{
    fn fn_output_ty(c: &mut CostomTypes) -> FuncOutput {
        FuncOutput::ServerStream {
            yield_ty: <G::Yield as TypeId>::ty(c),
            return_ty: <G::Return as TypeId>::ty(c),
        }
    }
}
