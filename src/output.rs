use async_trait::async_trait;
use databuf::Encode;
use std::{future::Future, io};

/// It defines the behavior for sending responses over a transport channel.
#[async_trait]
pub trait Transport {
    /// Sends a response in a unary request.
    async fn send_unary_response(&mut self, _: Box<[u8]>) -> std::io::Result<()>;
}

#[async_trait]
impl<T> Transport for T
where
    T: std::io::Write + Send,
{
    async fn send_unary_response(&mut self, buf: Box<[u8]>) -> std::io::Result<()> {
        self.write_all(&buf)
    }
}

/// It implemented by different types representing various output formats.
#[async_trait]
pub trait Output: crate::private::FnOutputType {
    /// It produces the output data and sends it over the specified transport.
    async fn produce<T>(self, _: &mut T) -> io::Result<()>
    where
        T: Transport + Unpin + Send;
}

#[async_trait]
impl<Fut> Output for Fut
where
    Fut: Future + Send,
    Fut::Output: Encode + frpc_message::TypeId,
{
    async fn produce<W>(self, transport: &mut W) -> io::Result<()>
    where
        W: Transport + Unpin + Send,
    {
        let mut buf = Vec::new();
        <Fut::Output as Encode>::encode::<{ crate::DATABUF_CONFIG }>(&self.await, &mut buf)?;
        transport.send_unary_response(buf.into_boxed_slice()).await
    }
}
