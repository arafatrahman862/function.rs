#![allow(warnings)]

mod context;
mod responce;
mod server;

pub use responce::*;

#[macro_export]
macro_rules! rpc {
    [$($func:path = $id:literal)*] => (mod rpc {
        use super::*;

        #[allow(dead_code)]
        pub const fn type_def() -> codegen::TypeDef {
            codegen::TypeDef {
                name: env!("CARGO_PKG_NAME"),
                version: env!("CARGO_PKG_VERSION"),
                funcs: &[$({
                    const TY: (&[codegen::Type], codegen::Type) = codegen::async_fn_ty(&$func);
                    codegen::Func { index: $id, name: stringify!($func), args: TY.0, retn: TY.1 }
                }),*],
            }
        }

        pub async fn execute<State>(
            mut reader: impl tokio::io::AsyncRead + std::marker::Unpin,
            writer: impl tokio::io::AsyncWrite,
            ctx: context::Ctx<State>,
        ) -> std::io::Result<()> {
            let mut buf = [0; 2];
            tokio::io::AsyncReadExt::read_exact(&mut reader, &mut buf).await?;

            match u16::from_le_bytes(buf) {
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

// async fn a(num: u8) -> u16 {
//     println!("{:?}", num);
//     123
// }

// rpc! {
//     a = 1
// }

// #[tokio::test]
// async fn test_name() {
//     let mut reader = [1u8, 0, 1, 0, 0, 42].as_slice();
//     let writer = vec![];

//     // println!("{:#?}", rpc::type_def());
//     rpc::execute(&mut reader, writer, context::Ctx { state: () }).await;
// }
