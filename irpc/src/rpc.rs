use crate::*;

#[cfg(debug_assertions)]
pub mod type_def {
    use super::*;
    #[derive(Debug, Clone)]
    pub struct Func {
        pub name: &'static str,
        pub args: Box<[Type]>,
        pub ret_ty: Type,
    }

    #[derive(Debug, Clone)]
    pub struct TypeDef {
        pub name: &'static str,
        pub version: &'static str,
        pub funcs: Vec<Func>,
    }
}

macro_rules! rpc {
    [$($h:path as $name: ident)*] => {
        mod rpc {
            use super::*;
            #[cfg(debug_assertions)]
            pub fn type_def() -> crate::type_def::TypeDef {
                crate::type_def::TypeDef {
                    name: env!("CARGO_PKG_NAME"),
                    version: env!("CARGO_PKG_VERSION"),
                    funcs: vec![$(crate::type_def::Func {
                        name: stringify!($name),
                        args: typegen::AsyncFnType::args_ty(&$h),
                        ret_ty: typegen::AsyncFnType::ret_ty(&$h)
                    }),*]
                }
            }

            pub async fn sarve(mut socket: tokio::net::TcpStream) -> Result<()> {
                use tokio::io::AsyncReadExt;
                loop {
                    let id = socket.read_u64_le().await?;
                    match id {
                        $(macros::hash_from_ident!($name) => {
                            Handler::call($h, ((), 0));
                        }),*
                        _ => return Ok(())
                    }
                }
            }
        }
    };
}