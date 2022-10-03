use crate::*;
use bin_layout::Encoder;
use codegen::GetType;
use std::result::Result;

pub trait Response:codegen::GetType {
    type Bytes: AsRef<[u8]>;
    type Error: std::fmt::Display;
    fn as_bytes(self) -> Result<Self::Bytes, Self::Error>;
}

impl<T: Encoder + codegen::GetType> Response for T {
    type Bytes = Vec<u8>;
    type Error = &'static str;
    fn as_bytes(self) -> Result<Self::Bytes, Self::Error> {
        let mut bytes = Vec::new();
        match self.encoder(&mut bytes) {
            Ok(_) => Ok(bytes),
            Err(_) => Err("Parse Error"),
        }
    }
}
