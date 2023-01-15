#![allow(warnings)]

mod context;
mod responce;
// mod server;

pub use responce::*;
use tokio::io;

pub struct RpcHeader {
    id: u16,
    data: Vec<u8>,
}

pub trait RpcChannel {}

#[macro_export]
macro_rules! rpc {
    [$($func:path = $id:literal)*] => (mod rpc {
        use super::*;

        #[allow(dead_code)]
        pub fn type_def() -> codegen::TypeDef {
            codegen::TypeDef {
                name: env!("CARGO_PKG_NAME").into(),
                version: env!("CARGO_PKG_VERSION").into(),
                funcs: vec![$({
                    let (args, retn) = codegen::async_fn_ty(&$func);
                    codegen::Func { index: $id, name: stringify!($func).into(), args, retn }
                }),*],
            }
        }

        pub async fn execute<State>(
            RpcHeader { id, data: _ } : RpcHeader,
            reader: impl tokio::io::AsyncRead + std::marker::Unpin,
            writer: impl tokio::io::AsyncWrite,
            ctx: context::Ctx<State>,
        ) -> std::io::Result<()> {
            match id {
                $($id => {
                    // let args = context::Parse::parse(ctx, &data).unwrap();
                    // std_trait::FnOnce::call_once($func, args).await;
                }),*
                _=> {}
            }
            return Ok(());
        }
    });
}

// ---------------------------------------------------------------------

async fn a(num: u8) -> u8 {
    123
}

rpc! {
    a = 1
}

#[test]
fn test_name() {
    // println!("{:#?}", rpc::type_def());
}
