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
        match self.0 {
            Struct(s) => f.write_str(&s.name),
            Func(func) => f.write_str(&func.name),
            Union(u) => f.write_str(&u.name),
            Enum(e) => f.write_str(&e.name),

            Any => f.write_str("any"),
            Tuple(tys) => match tys.is_empty() {
                true => f.write_str("null"),
                false => f.debug_list().entries(tys).finish(),
            },
            Bool => f.write_str("boolean"),
            String => f.write_str("string"),
            U8 | U16 | U32 | I8 | I16 | I32 | F32 | F64 => f.write_str("number"),
            I64 | U64 | I128 | U128 => f.write_str("bigint"),

            Vec(ty) => write!(f, "Array<{:?}>", TypeOf(ty)),
            Option(ty) => write!(f, "{:?} | undefined", TypeOf(ty)),
            Result(ty) => TypeOf(&ty.0).fmt(f),
        }
    }
}