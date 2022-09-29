#![allow(warnings)]
mod context;
mod handler;
mod response;
mod rpc;

pub use handler::Handler;
pub use response::Response;
pub use rpc::type_def;

use bin_layout::{Decoder, Encoder};
use std::{future::Future, io::Result};
use tokio::net::{TcpListener, TcpStream};
use typegen::{AsyncFnType, Type};

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
    async fn main() -> Result<()> {
        let listener = TcpListener::bind("0.0.0.0:1234").await?;
        serve(listener, |stream| async {});
        Ok(())
    }
}
