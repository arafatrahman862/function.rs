use async_gen::GeneratorState;
use async_trait::async_trait;
use databuf::Encode;
use std::{
    future::{poll_fn, Future},
    io,
};

use crate::AsyncGenerator;

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

#[allow(missing_docs)]
pub struct SSE<G>(pub G);

// ---------------------------------------------------------

#[async_trait]
impl<G> Output for SSE<G>
where
    G: AsyncGenerator + Send,
{
    async fn produce<W>(self, _: &mut W) -> io::Result<()>
    where
        W: Transport + Unpin + Send,
    {
        let mut gen = std::pin::pin!(self.0);
        loop {
            match poll_fn(|cx| AsyncGenerator::poll_resume(gen.as_mut(), cx)).await {
                GeneratorState::Yielded(val) => {
                    let mut buf = vec![0];
                    Encode::encode::<{ crate::DATABUF_CONFIG }>(&val, &mut buf)?;
                    todo!()
                }
                GeneratorState::Complete(val) => {
                    let mut buf = vec![1];
                    Encode::encode::<{ crate::DATABUF_CONFIG }>(&val, &mut buf)?;
                    todo!();
                    break Ok(());
                }
            }
        }
    }
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
        Encode::encode::<{ crate::DATABUF_CONFIG }>(&self.await, &mut buf)?;
        transport.send_unary_response(buf.into_boxed_slice()).await
    }
}
