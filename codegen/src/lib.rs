#![allow(warnings)]

use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Debug},
    future::Future,
};
pub use typegen::{GetType, Type};

pub fn async_fn_ty<Func, Args, Ret>(_: &Func) -> (Vec<Type>, Type)
where
    Func: std_trait::FnOnce<Args>,
    Func::Output: Future<Output = Ret>,
    Args: GetType,
    Ret: GetType,
{
    let Type::Tuple(types) = Args::ty() else { unreachable!() };
    (types, Ret::ty())
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Func {
    pub index: u16,
    pub name: String,
    pub args: Vec<Type>,
    pub retn: Type,
}

// impl Debug for Func {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_map()
//             .entry(&"index", &self.index)
//             .entry(&"name", &self.name)
//             .entry(&"args", &self.args)
//             .entry(&"retn", &FmtTy(self.retn.clone()))
//             .finish()
//     }
// }

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TypeDef {
    pub name: String,
    pub version: String,
    pub funcs: Vec<Func>,
}

// impl fmt::Display for TypeDef {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_map()
//             .entry(&"name", &self.name)
//             .entry(&"version", &self.version)
//             .entry(&"functions", &self.funcs)
//             .finish()
//     }
// }
