#![allow(warnings)]
mod context;
mod handler;
mod response;

pub use handler::Handler;
pub use response::Response;

use bin_layout::Decoder;
use std::{future::Future, io::Result};
use tokio::net::{TcpListener, TcpStream};

async fn serve<T>(listener: TcpListener, service: impl Fn(TcpStream) -> T) -> Result<()>
where
    T: Future + Send + 'static,
    T::Output: Send,
{
    loop {
        let (stream, addr) = listener.accept().await?;
        tokio::spawn(service(stream));
    }
}

mod tests {
    use super::*;

    mod handlers {
        use super::*;
        pub async fn intro(name: String) -> impl Response {
            format!("Hello! {name}")
        }

        pub async fn index() -> impl Response {
            "Hello, World!"
        }
    }

    #[test]
    fn get_types() {
        #[cfg(debug_assertions)]
        {
            static GEN_TY: std::sync::Once = std::sync::Once::new();
            GEN_TY.call_once(|| {
                let mut path = std::path::PathBuf::from(format!(
                    "{}\\target\\types",
                    env!("CARGO_MANIFEST_DIR")
                ));
                std::fs::create_dir_all(&path).unwrap();

                let version = env!("CARGO_PKG_VERSION");
                path.push(format!(
                    "{}_v{}.ty",
                    env!("CARGO_PKG_NAME"),
                    version.replace(".", "_")
                ));

                let mut file = std::fs::File::options()
                    .create(true)
                    .write(true)
                    .read(true)
                    .open(path)
                    .unwrap();

                use typegen::AsyncFnType;

                // #[derive(bin_layout::Encoder)]
                struct TypeData {
                    version: String,
                    funcs: Vec<(String,)>
                }

                std::io::Write::write(&mut file, b"");
            });
        }
    }

    async fn main() -> Result<()> {
        let listener = TcpListener::bind("0.0.0.0:1234").await?;
        serve(listener, |stream| async {
            {
                static GEN_TY: std::sync::Once = std::sync::Once::new();
                GEN_TY.call_once(|| {});
            }
        });
        Ok(())
    }
}
