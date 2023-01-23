use super::*;

macro_rules! impl_for {
    [$($ty:tt),*] => {$(impl Message for $ty { fn ty(_: &mut Context) -> Type { Type::$ty } })*};
}

impl_for!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char, String
);

impl<T: Message, const N: usize> Message for [T; N] {
    fn ty(def: &mut Context) -> Type {
        Type::Array {
            len: N,
            ty: Box::new(T::ty(def)),
        }
    }
}

impl<T: Message> Message for Option<T> {
    fn ty(def: &mut Context) -> Type {
        Type::Option(Box::new(T::ty(def)))
    }
}

impl<T: Message, E: Message> Message for Result<T, E> {
    fn ty(def: &mut Context) -> Type {
        Type::Result(Box::new((T::ty(def), E::ty(def))))
    }
}

impl Message for &str {
    fn ty(_: &mut Context) -> Type {
        Type::str
    }
}

// impl<T: Message> Message for &[T] {
//     fn ty() -> Type {
//         Type::Slice(Box::new(T::ty()))
//     }
// }