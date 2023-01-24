use async_trait::async_trait;
use databuf::Encoder;
use frpc_message::Message;
use std::io::Result;
use tokio::io::{AsyncWrite, AsyncWriteExt};

#[async_trait]
pub trait Output: Message {
    async fn write<W>(&self, _: &mut W) -> Result<()>
    where
        W: AsyncWrite + Unpin + Send;
}

#[async_trait]
impl<T: Encoder + Message + Sync> Output for T {
    async fn write<W>(&self, writer: &mut W) -> Result<()>
    where
        W: AsyncWrite + Unpin + Send,
    {
        let bytes = T::encode(self);
        writer.write_all(&bytes).await
    }
}
