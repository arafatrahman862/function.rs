use super::*;

macro_rules! impl_num { [$($ty:tt),*] => {$( impl GetType for $ty { #[inline] fn get_ty() -> Type { Type::$ty } } )*}; }
impl_num!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char, str,
    String
);

impl<T: GetType, const N: usize> GetType for [T; N] {
    #[inline]
    fn get_ty() -> Type {
        Type::Array {
            len: N,
            ty: Box::new(T::get_ty()),
        }
    }
}

impl<T: GetType> GetType for [T] {
    #[inline]
    fn get_ty() -> Type {
        Type::Slice(Box::new(T::get_ty()))
    }
}

impl<T: GetType> GetType for Option<T> {
    #[inline]
    fn get_ty() -> Type {
        Type::Option(Box::new(T::get_ty()))
    }
}

impl<T: GetType, E: GetType> GetType for Result<T, E> {
    #[inline]
    fn get_ty() -> Type {
        Type::Result(Box::new((T::get_ty(), E::get_ty())))
    }
}