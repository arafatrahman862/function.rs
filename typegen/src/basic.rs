use super::*;

macro_rules! impl_num {
    [$($ty:tt),*] => {$( impl GetType for $ty { const TYPE: Type = Type::$ty; } )*};
}

impl_num!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char, str,
    String
);

impl<T: GetType, const N: usize> GetType for [T; N] {
    const TYPE: Type = Type::Array {
        len: N,
        ty: &T::TYPE,
    };
}

impl<T: GetType> GetType for [T] {
    const TYPE: Type = Type::Slice(&T::TYPE);
}

impl<T: GetType> GetType for Option<T> {
    const TYPE: Type = Type::Option(&T::TYPE);
}

impl<T: GetType, E: GetType> GetType for Result<T, E> {
    const TYPE: Type = Type::Result(&(T::TYPE, E::TYPE));
}
