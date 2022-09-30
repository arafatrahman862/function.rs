#![allow(warnings)]

mod rust_codegen;
pub use typegen::*;

#[derive(Debug, Clone)]
pub struct Func {
    pub name: String,
    pub args: Box<[Type]>,
    pub ret_ty: Type,
}

#[derive(Debug, Clone)]
pub struct TypeDef {
    pub name: String,
    pub version: String,
    pub funcs: Vec<Func>,
}

