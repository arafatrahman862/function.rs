use super::*;

// impl Message for std::convert::Infallible {
//     fn ty() -> Ty {
//         Ty::Never
//     }
// }

impl<T: Message> Message for &T {
    fn ty(def: &mut Context) -> Ty {
        T::ty(def)
    }
}

impl<T: Message> Message for Box<T> {
    fn ty(def: &mut Context) -> Ty {
        T::ty(def)
    }
}

macro_rules! impl_for_typles {
    [$(($($ty: ident),*)),*]  => ($(
        impl<$($ty),*> Message for ($($ty,)*)
        where
            $($ty: Message),*
        {
            fn ty(_ctx: &mut Context) -> Ty {
                Ty::Tuple(vec![$($ty::ty(_ctx)),*])
            }
        }
    )*);
}

impl_for_typles!(
    (),
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
