use super::*;
use std::future::Future;

pub trait AsyncFnType<Args> {
    fn args_ty(&self) -> Box<[Type]>;
    fn ret_ty(&self) -> Type;
}

macro_rules! impl_for_typles {
    [$(($($ty: ident),*)),*]  => ($(
        impl<Func, Fut, Ret, $($ty),*> AsyncFnType<($($ty),*,)> for Func
        where
            Ret: GetType,
            Fut: Future<Output = Ret>,
            Func: FnOnce($($ty),*) -> Fut,
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

impl<Func, Fut, Ret> AsyncFnType<()> for Func
where
    Ret: GetType,
    Fut: Future<Output = Ret>,
    Func: FnOnce() -> Fut,
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

// fn get_fn_ty<Func, Args, Ret>(_: Func) -> (Box<[Type]>, Type)
// where
//     Func: FnOnce<Args, Output = Ret>,
//     Args: GetType,
//     Ret: GetType,
// {
//     (
//         match Args::get_ty() {
//             Type::Tuple(types) => types,
//             _ => unreachable!(),
//         },
//         Ret::get_ty(),
//     )
// }