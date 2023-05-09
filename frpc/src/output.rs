use async_trait::async_trait;
use databuf::Encode;
use std::io::Result;
use tokio::io::{AsyncWrite, AsyncWriteExt};

#[async_trait]
pub trait Output {
    async fn write<W>(&self, _: &mut W) -> Result<()>
    where
        W: AsyncWrite + Unpin + Send;
}

#[async_trait]
impl<T: Encode + Sync> Output for T {
    async fn write<W>(&self, writer: &mut W) -> Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        let bytes = T::to_bytes::<{ crate::DATABUF_CONFIG }>(self);
        writer.write_all(&bytes).await
    }
}
