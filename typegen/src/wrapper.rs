use super::*;

impl<T: GetType + ?Sized> GetType for &T {
    fn ty() -> Type {
        T::ty()
    }
}

impl<T: GetType + ?Sized> GetType for &mut T {
    fn ty() -> Type {
        T::ty()
    }
}

impl GetType for std::convert::Infallible {
    fn ty() -> Type {
        Type::Never
    }
}

macro_rules! impl_for_typles {
    [$(($($ty: ident),*)),*]  => ($(
        impl<$($ty),*> GetType for ($($ty,)*)
        where
            $($ty: GetType),*
        {
            // const TYPE: Type = Type::Tuple(&[$($ty::ty()),*]);
            fn ty() -> Type {
                // Type::Never
                Type::Tuple(vec![$($ty::ty()),*])
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
