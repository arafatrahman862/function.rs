use super::*;
pub trait FnType<Args> {
    fn args_ty(&self) -> Box<[Type]>;
    fn ret_ty(&self) -> Type;
}

macro_rules! impl_for_typles {
    [$(($($ty: ident),*)),*]  => ($(
        impl<Func, Ret, $($ty),*> FnType<($($ty),*,)> for Func
        where
            Func: FnOnce($($ty),*) -> Ret,
            Ret: GetType,
            $($ty: GetType),*
        {
            #[inline] fn args_ty(&self) -> Box<[Type]> {
                match <($($ty),*,)>::get_ty() {
                    Type::Tuple(types) => types,
                    _ => unsafe { std::hint::unreachable_unchecked() }
                }
            }
            #[inline] fn ret_ty(&self) -> Type {
                Ret::get_ty()
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
impl<Func, Ret> FnType<()> for Func
where
    Ret: GetType,
    Func: FnOnce() -> Ret,
{
    #[inline]
    fn args_ty(&self) -> Box<[Type]> {
        Box::new([])
    }
    #[inline]
    fn ret_ty(&self) -> Type {
        Ret::get_ty()
    }
}
