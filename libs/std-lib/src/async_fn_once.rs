use std::future::Future;

pub trait AsyncFnOnce<Args> {
    type Output;
    type Future: Future<Output = Self::Output> + Send;
    fn call_once(self, _: Args) -> Self::Future;
}

impl<Func, Args> AsyncFnOnce<Args> for Func
where
    Func: crate::fn_once::FnOnce<Args>,
    Func::Output: Future + Send,
{
    type Output = <Func::Output as Future>::Output;
    type Future = Func::Output;

    fn call_once(self, args: Args) -> Self::Future {
        Func::call_once(self, args)
    }
}
