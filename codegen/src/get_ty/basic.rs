use crate::*;

macro_rules! impl_for { [$($ty:tt),*] => {$( 
    impl GetType for $ty { fn ty() -> Type { Type::$ty } }
    // impl Resource<'_> for $ty {}
)*}; }
impl_for!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char, str,
    String
);


impl<T: GetType, const N: usize> GetType for [T; N] {
    fn ty() -> Type {
        Type::Array {
            len: N,
            ty: Box::new(T::ty()),
        }
    }
}

impl<T: GetType> GetType for [T] {
    fn ty() -> Type {
        Type::Slice(Box::new(T::ty()))
    }
}

impl<T: GetType> GetType for Option<T> {
    fn ty() -> Type {
        Type::Option(Box::new(T::ty()))
    }
}

impl<T: GetType, E: GetType> GetType for Result<T, E> {
    fn ty() -> Type {
        Type::Result(Box::new((T::ty(), E::ty())))
    }
}
