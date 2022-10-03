use crate::*;
use std::{convert::Infallible, result::Result};

pub trait Function<Args> {
    type Output;
    fn call(self, _: Args) -> Self::Output;
}

impl<Func, Ret> Function<()> for Func
where
    Func: FnOnce() -> Ret,
{
    type Output = Ret;

    #[inline]
    fn call(self, _: ()) -> Self::Output {
        self()
    }
}

macro_rules! impl_for_typles {
    [$(($($i: tt; $ty: ident),*)),*]  => ($(
        impl<Func, Ret, $($ty),*> Function<($($ty),*,)> for Func
        where
            Func: FnOnce($($ty),*) -> Ret,
        {
            type Output = Ret;
            #[inline] fn call(self, args: ($($ty),*,)) -> Self::Output {
                self($(args.$i),*)
            }
        }
    )*);
}

impl_for_typles!(
    (0; T0),
    (0; T0, 1; T1),
    (0; T0, 1; T1, 2; T2),
    (0; T0, 1; T1, 2; T2, 3; T3),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4, 5; T5),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4, 5; T5, 6; T6),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4, 5; T5, 6; T6, 7; T7),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4, 5; T5, 6; T6, 7; T7, 8; T8),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4, 5; T5, 6; T6, 7; T7, 8; T8, 9; T9),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4, 5; T5, 6; T6, 7; T7, 8; T8, 9; T9, 10; T10),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4, 5; T5, 6; T6, 7; T7, 8; T8, 9; T9, 10; T10, 11; T11),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4, 5; T5, 6; T6, 7; T7, 8; T8, 9; T9, 10; T10, 11; T11, 12; T12),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4, 5; T5, 6; T6, 7; T7, 8; T8, 9; T9, 10; T10, 11; T11, 12; T12, 13; T13),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4, 5; T5, 6; T6, 7; T7, 8; T8, 9; T9, 10; T10, 11; T11, 12; T12, 13; T13, 14; T14),
    (0; T0, 1; T1, 2; T2, 3; T3, 4; T4, 5; T5, 6; T6, 7; T7, 8; T8, 9; T9, 10; T10, 11; T11, 12; T12, 13; T13, 14; T14, 15; T15)
);