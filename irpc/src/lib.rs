#![allow(warnings)]

mod func;
mod response;
mod rpc;

pub use func::Function;
pub use response::Response;

use bin_layout::{Decoder, Encoder};
use std::{future::Future, io::Result};
use tokio::{
    io::BufReader,
    net::{TcpListener, TcpStream},
};

type DynErr = Box<dyn std::error::Error + Send + Sync>;

async fn accept_connection<T>(listener: TcpListener, service: impl Fn(TcpStream) -> T) -> Result<()>
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
        accept_connection(listener, |stream| async {});
        Ok(())
    }
}
