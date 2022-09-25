#![allow(warnings)]
mod context;
mod handler;
mod response;

pub use handler::Handler;
pub use response::Response;

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
    async fn main() -> Result<()> {
        let listener = TcpListener::bind("0.0.0.0:1234").await?;
        serve(listener, |stream| async {
            // stream.read
        });
        Ok(())
    }
}
