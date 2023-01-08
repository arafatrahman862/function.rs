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
            mut reader: impl tokio::io::AsyncRead + std::marker::Unpin,
            writer: impl tokio::io::AsyncWrite,
            ctx: context::Ctx<State>,
        ) -> std::io::Result<()> {
            let mut buf = [0; 5];
            tokio::io::AsyncReadExt::read_exact(&mut reader, &mut buf).await?;

            let [b0, b1, b2, b3, b4] = buf;

            let id = u16::from_le_bytes([b0, b1]);

            let data_len: usize = u32::from_le_bytes([b2, b3, b4, 0]).try_into().unwrap();
            let mut data = vec![0; data_len];
            tokio::io::AsyncReadExt::read_exact(&mut reader, &mut data).await?;

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

async fn a(num: u8) -> u8 {
    println!("{:?}", num);
    123
}

rpc! {
    a = 1
}
