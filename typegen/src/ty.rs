use crate::Type;
trait Ty {
    fn get_ty() -> Type;
    fn ty(&self) -> Type {
        Self::get_ty()
    }
}

macro_rules! impls {
    [$tr:tt for $($ty:ty : $out: expr);*] => {$( impl $tr for $ty { fn get_ty() -> Type { $out } } )*};
}

impls!(Ty for
    u8: Type::U8;
    u16: Type::U16;
    u32: Type::U32;
    u64: Type::U64;
    u128: Type::U128;

    i8: Type::I8;
    i16: Type::I16;
    i32: Type::I32;
    i64: Type::I64;
    i128: Type::I128;

    f32: Type::F32;
    f64: Type::F64;

    bool: Type::Bool;
    String: Type::String;
    (): Type::Tuple(Vec::new())
);

impl<T: Ty, const N: usize> Ty for [T; N] {
    fn get_ty() -> Type {
        T::get_ty().arr(N)
    }
}
impl<T: Ty> Ty for Vec<T> {
    fn get_ty() -> Type {
        T::get_ty().list()
    }
}
impl<T: Ty> Ty for Option<T> {
    fn get_ty() -> Type {
        T::get_ty().optional()
    }
}
impl<T: Ty, E: Ty> Ty for Result<T, E> {
    fn get_ty() -> Type {
        Type::result(T::get_ty(), E::get_ty())
    }
}