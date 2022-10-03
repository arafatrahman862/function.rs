use crate::*;

macro_rules! rpc {
    [$($func:path = $id: literal)*] => {
        mod rpc {
            use super::*;
            pub fn type_def() -> codegen::TypeDef {
                codegen::TypeDef {
                    name: env!("CARGO_PKG_NAME").into(),
                    version: env!("CARGO_PKG_VERSION").into(),
                    funcs: vec![$(codegen::Func {
                        name: stringify!($func).into(),
                        args: codegen::AsyncFnType::args_ty(&$func),
                        ret_ty: codegen::AsyncFnType::ret_ty(&$func)
                    }),*]
                }
            }
            pub async fn sarve(mut stream: &mut tokio::net::TcpStream) -> Result<()> {
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
                            Function::call($func, args).await;
                        }),*
                        _ => return Ok(())
                    }
                }
            }
        }
    };
}

async fn a(a: u8) -> impl Response {
    0
}
async fn b(a: u8, u: u8) {}
async fn c(a: u8, u: u8) {}

rpc! {
    // a = 1
    // b = 2
    // c = 2
}
