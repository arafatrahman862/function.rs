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

macro_rules! impl_for_typles {
    [$(($($ty: ident),*)),*]  => ($(
        impl<'de, Func, Fut, Ret, $($ty),*> Handler<($($ty),*,)> for Func
        where
            Ret: Response,
            Fut: Future<Output = Ret>,
            Func: FnOnce($($ty),*) -> Fut,
            $($ty: Decoder<'de>),*
        {
            type Output = Ret;
            type IntoFuture = Fut;

            #[inline] fn call(self, data: ()) -> Self::IntoFuture {
                todo!()
            }
        }
    )*);
}

impl_for_typles!(
    (T1),
    (T1, T2),
    (T1, T2, T3),
    (T1, T2, T3, T4),
    (T1, T2, T3, T4, T5),
    (T1, T2, T3, T4, T5, T6),
    (T1, T2, T3, T4, T5, T6, T7),
    (T1, T2, T3, T4, T5, T6, T7, T8),
    (T1, T2, T3, T4, T5, T6, T7, T8, T9),
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10),
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11),
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12),
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13),
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14),
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15),
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
);