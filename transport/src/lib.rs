mod tcp;

use async_trait::async_trait;
use std::{
    io::Result,
    net::SocketAddr
};
use tokio::io::AsyncRead;

#[async_trait]
pub trait Listener {
    type Stream: AsyncRead + Send + Unpin;
    async fn accept(&self) -> Result<(Self::Stream, SocketAddr)>;
}