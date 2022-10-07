use crate::*;

#[macro_export]
macro_rules! rpc {
    [$($func:path = $id: literal)*] => {
        mod rpc {
            use super::*;
            fn async_fn_ty<Func, Args, Ret>(_: &Func) -> (Box<[codegen::Type]>, codegen::Type)
            where
                Func: std_trait::FnOnce<Args>,
                Func::Output: Future<Output = Ret>,
                Args: codegen::GetType,
                Ret: codegen::GetType,
            {
                (
                    match Args::get_type() {
                        codegen::Type::Tuple(types) => types,
                        _ => unreachable!(),
                    },
                    Ret::get_type(),
                )
            }

            pub fn type_def() -> codegen::TypeDef {
                codegen::TypeDef {
                    name: env!("CARGO_PKG_NAME").into(),
                    version: env!("CARGO_PKG_VERSION").into(),
                    funcs: vec![$({
                        let (args, ret_ty) = async_fn_ty(&$func);
                        codegen::Func { name: stringify!($func).into(), args, ret_ty }
                    }),*]
                }
            }
            pub async fn sarve<T>(stream: &mut T) -> Result<()>
            where
                T: AsyncRead + AsyncWrite + Unpin
            {
                use tokio::io::AsyncReadExt;
                loop {
                    let mut buf = [0; 5];
                    stream.read_exact(&mut buf).await?;

                    let [b0, b1, b2, b3, b4] = buf;
                    let id = u16::from_le_bytes([b0, b1]);
                    let data_len: usize = u32::from_le_bytes([b2, b3, b4, 0]).try_into().unwrap();

                    let mut data = vec![0; data_len];
                    stream.read_exact(&mut data).await?;

                    match id {
                        $($id => {
                            let args = Decoder::decode(&data).unwrap();
                            std_trait::FnOnce::call_once($func, args).await;
                        }),*
                        _ => return Ok(())
                    }
                }
            }
        }
    };
}
