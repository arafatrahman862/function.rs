use std::future::Future;

pub trait AsyncFnOnce<Args> {
    type Output;
    type Future: Future<Output = Self::Output> + Send;
    fn call_once(self, _: Args) -> Self::Future;
}

impl<Func, Args, Fut, Ret> AsyncFnOnce<Args> for Func
where
    Func: crate::fn_once::FnOnce<Args, Output = Fut>,
    Fut: Future<Output = Ret> + Send,
{
    type Output = Ret;
    type Future = Fut;

    fn call_once(self, args: Args) -> Self::Future {
        self.call_once(args)
    }
}
