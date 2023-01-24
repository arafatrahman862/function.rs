pub mod input;
pub mod output;
pub mod fn_once;
pub mod definition;

pub use tokio;
pub use databuf;
pub use frpc_message;

pub use frpc_macro::Message;
// pub use frpc_message::Message;

pub use databuf::{Decoder, Encoder};

#[macro_export]
macro_rules! procedure {
    [$($func:path = $id:literal)*] => (mod procedure {
        use super::*;

        #[allow(dead_code)]
        pub fn type_def() -> $crate::definition::TypeDef {
            let mut ctx = $crate::frpc_message::Context::default();
            let funcs = vec![
                $({
                    let (args, retn) = $crate::definition::async_fn_ty(&$func, &mut ctx);
                    $crate::definition::Func { index: $id, name: stringify!($func).into(), args, retn }
                }),*
            ];
            $crate::definition::TypeDef {
                name: env!("CARGO_PKG_NAME").into(),
                version: env!("CARGO_PKG_VERSION").into(),
                description: env!("CARGO_PKG_DESCRIPTION").into(),
                ctx,
                funcs,
            }
        }

        pub async fn execute<W>(id: u16, data: Vec<u8>, writer: &mut W) -> ::std::io::Result<()>
        where
            W: $crate::tokio::io::AsyncWrite + ::std::marker::Unpin + ::std::marker::Send,
        {
            match id {
                $($id => {
                    let args = $crate::Decoder::decode(&data).unwrap();
                    let output = $crate::fn_once::FnOnce::call_once(user, args).await;
                    $crate::output::Output::write(&output, writer).await
                }),*
                _ => {
                    return ::std::result::Result::Err(::std::io::Error::new(
                        ::std::io::ErrorKind::AddrNotAvailable,
                        "Unknown request id",
                    ))
                }
            }
        }
    });
}
