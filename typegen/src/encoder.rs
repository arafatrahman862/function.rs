use super::*;
use bin_layout::{Decoder, Encoder};

impl Encoder for Type {
    fn encoder(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        self.ty_id().encoder(w)?;
        match self {
            Type::Option(ty) => ty.encoder(w),
            Type::Result(ty) => ty.encoder(w),
            Type::Slice(ty) => ty.encoder(w),
            Type::Tuple(fields) => fields.encoder(w),
            Type::TupleStruct { name, fields } => {
                name.encoder(w)?;
                fields.encoder(w)
            }
            Type::Struct { name, fields } | Type::Enum { name, fields } => {
                name.encoder(w)?;
                fields.encoder(w)
            }
            Type::Array { len, ty } => {
                u32::try_from(*len).unwrap().encoder(w)?;
                ty.encoder(w)
            }
            Type::Set { variant, ty } => {
                variant.ty_id().encoder(w)?;
                ty.encoder(w)
            }
            Type::Map { variant, ty } => {
                variant.ty_id().encoder(w)?;
                ty.encoder(w)
            }
            _ => Ok(()),
        }
    }
}

impl Decoder<'_> for Type {
    fn decoder(r: &mut &[u8]) -> Result<Self, Box<(dyn std::error::Error + Send + Sync)>> {
        Ok(match u8::decoder(r)? {
            0 => Type::u8,
            1 => Type::u16,
            2 => Type::u32,
            3 => Type::u64,
            4 => Type::u128,
            5 => Type::usize,
            6 => Type::i8,
            7 => Type::i16,
            8 => Type::i32,
            9 => Type::i64,
            10 => Type::i128,
            11 => Type::isize,
            12 => Type::f32,
            13 => Type::f64,
            14 => Type::bool,
            15 => Type::char,
            16 => Type::str,
            17 => Type::String,
            18 => Type::Option(Decoder::decoder(r)?),
            19 => Type::Result(Decoder::decoder(r)?),
            20 => Type::Slice(Decoder::decoder(r)?),
            21 => Type::Tuple(Decoder::decoder(r)?),
            22 => Type::TupleStruct {
                name: Decoder::decoder(r)?,
                fields: Decoder::decoder(r)?,
            },
            23 => Type::Struct {
                name: Decoder::decoder(r)?,
                fields: Decoder::decoder(r)?,
            },
            24 => Type::Enum {
                name: Decoder::decoder(r)?,
                fields: Decoder::decoder(r)?,
            },
            25 => Type::Array {
                len: u32::decoder(r)?.try_into()?,
                ty: Decoder::decoder(r)?,
            },
            26 => Type::Set {
                variant: u8::decoder(r)?.try_into()?,
                ty: Decoder::decoder(r)?,
            },
            27 => Type::Map {
                variant: u8::decoder(r)?.try_into()?,
                ty: Decoder::decoder(r)?,
            },
            id => {
                return Err(
                    format!("Can't create `{}` from `u8`: {id}", type_name::<Self>()).into(),
                )
            }
        })
    }
}
