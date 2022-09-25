use crate::*;

pub trait Handler<Args> {
    type Output: Response;
    type IntoFuture: Future<Output = Self::Output>;
    
    fn call(self, data: ()) -> Self::IntoFuture;
}

impl<Func, Fut, Ret> Handler<()> for Func
where
    Ret: Response,
    Fut: Future<Output = Ret>,
    Func: FnOnce() -> Fut,
{
    type Output = Ret;
    type IntoFuture = Fut;

    fn call(self, data: ()) -> Self::IntoFuture {
        self()
    }
}
