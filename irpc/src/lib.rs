#![allow(warnings)]

mod response;
mod rpc;

pub use response::Response;
pub use rpc::*;

use bin_layout::Decoder;
use std::{future::Future, io::Result};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader},
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

async fn get_transport(stream: TcpStream, capacity: usize) -> Result<impl AsyncRead + AsyncWrite> {
    let mut buf_reader = BufReader::with_capacity(capacity, stream);
    Ok(buf_reader)
}

mod tests {
    // use std_trait::FnOnce;
    use codegen::Type;

    use super::*;
    async fn a(a: u8) -> u8 {
        0
    }

    rpc! {
        a = 1
    }

    async fn main() -> Result<()> {
        let listener = TcpListener::bind("0.0.0.0:1234").await?;
        accept_connection(listener, |stream| async {
            let mut stream = get_transport(stream, 8192).await.unwrap();
            rpc::sarve(&mut stream).await.unwrap();
        })
        .await
    }
}
