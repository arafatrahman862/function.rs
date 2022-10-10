use super::*;

impl<T: GetType + ?Sized> GetType for &T {
    const TYPE: Type = T::TYPE;
}

impl<T: GetType + ?Sized> GetType for &mut T {
    const TYPE: Type = T::TYPE;
}

impl GetType for std::convert::Infallible {
    const TYPE: Type = Type::Never;
}

trait Tys {
    const TYS: &'static [Type];
}

macro_rules! impl_for_typles {
    [$(($($ty: ident),*)),*]  => ($(
        impl<$($ty),*> Tys for ($($ty,)*)
        where
            $($ty: GetType),*
        {
            const TYS: &'static [Type] =  &[$($ty::TYPE),*];
        }

        impl<$($ty),*> GetType for ($($ty,)*)
        where
            $($ty: GetType),*
        {
            const TYPE: Type = Type::Tuple(Self::TYS);
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
