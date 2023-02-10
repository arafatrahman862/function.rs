pub mod fn_once;
pub mod output;
pub mod util;

pub use frpc_macros::Message;
pub use frpc_message;

#[macro_export]
macro_rules! procedure {
    [$($func:path = $id:literal)*] => (mod procedure {
        use super::*;

        #[allow(dead_code)]
        pub fn type_def() -> $crate::frpc_message::TypeDef {
            let mut ctx = $crate::frpc_message::Context::default();
            let funcs = vec![
                $({
                    let (args, retn) = $crate::util::async_fn_ty(&$func, &mut ctx);
                    $crate::frpc_message::Func { index: $id, path: stringify!($func).into(), args, retn }
                }),*
            ];
            $crate::frpc_message::TypeDef {
                name: env!("CARGO_PKG_NAME").into(),
                version: env!("CARGO_PKG_VERSION").into(),
                description: env!("CARGO_PKG_DESCRIPTION").into(),
                ctx,
                funcs,
            }
        }

        pub async fn execute<W>(id: u16, data: Vec<u8>, writer: &mut W) -> ::std::io::Result<()>
        where
            W: ::tokio::io::AsyncWrite + ::std::marker::Unpin + ::std::marker::Send,
        {
            match id {
                $($id => {
                    let args = ::databuf::Decoder::decode(&data).unwrap();
                    let output = $crate::fn_once::FnOnce::call_once(user, args).await;
                    $crate::output::Output::write(&output, writer).await
                }),*
                _ => {
                    return ::std::result::Result::Err(::std::io::Error::new(
                        ::std::io::ErrorKind::AddrNotAvailable,
                        "Unknown id",
                    ))
                }
            }
        }
    });
}
