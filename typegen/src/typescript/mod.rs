use crate::*;

mod enumurate;
mod function;
mod structure;
mod union_ty;

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Struct(s) => s.fmt(f),
            Self::Func(func) => func.fmt(f),
            Self::Union(u) => u.fmt(f),
            Self::Enum(e) => e.fmt(f),
            _ => TypeOf(self).fmt(f),
        }
    }
}
pub struct TypeOf<'a>(pub &'a Type);

impl Debug for TypeOf<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Type::*;
        f.write_str(match self.0 {
            Enum(e) => &e.name,
            Union(u) => &u.name,
            Struct(s) => &s.name,
            Func(fc) => &fc.name,

            Any => "any",
            Bool => "boolean",
            String => "string",
            I64 | U64 | I128 | U128 => "bigint",
            U8 | U16 | U32 | I8 | I16 | I32 | F32 | F64 => "number",

            Vec(ty) => return write!(f, "Array<{:?}>", TypeOf(ty)),
            Option(ty) => return write!(f, "{:?} | undefined", TypeOf(ty)),
            Result(ty) => return  TypeOf(&ty.0).fmt(f),
            Tuple(tys) => match tys.is_empty() {
                true => "null",
                false => return  f.debug_list().entries(tys).finish(),
            },
            Array(ty, _len) => match ty.as_ref() {
                U8 => "Uint8Array",
                U16 => "Uint16Array",
                U32 => "Uint32Array",
                U64 => "BigUint64Array",

                I8 => "Int8Array",
                I16 => "Int16Array",
                I32 => "Int32Array",
                I64 => "BigInt64Array",

                F32 => "Float32Array",
                F64 => "Float64Array",
                ty => panic!("Invalid number type: `{:?}`", ty)
            },
        })
    }
}