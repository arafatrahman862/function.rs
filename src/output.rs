use super::*;
use async_gen::GeneratorState;
use async_trait::async_trait;
use databuf::Encode;
use frpc_message::TypeId;
use std::{
    future::{poll_fn, Future},
    io,
    pin::pin,
};

/// It defines the behavior for sending responses over a transport channel.
#[async_trait]
pub trait Transport {
    #[allow(missing_docs)]

    /// Sends a response in a unary request.
    async fn send_unary_response(&mut self, _: Box<[u8]>) -> io::Result<()>;

    #[allow(missing_docs)]
    fn create_server_stream(&mut self) {}
}

/// It implemented by different types representing various output formats.
#[async_trait]
pub trait Output: crate::output_type::OutputType {
    /// It produces the output data and sends it over the specified transport.
    async fn produce<TR>(self, _: &mut TR) -> io::Result<()>
    where
        TR: Transport + Send;
}

#[async_trait]
impl<T> Output for Return<T>
where
    T: Send + Encode + TypeId,
{
    async fn produce<TR>(self, transport: &mut TR) -> io::Result<()>
    where
        TR: Transport + Send,
    {
        let mut buf = Vec::new();
        Encode::encode::<{ crate::DATABUF_CONFIG }>(&self.0, &mut buf)?;
        transport.send_unary_response(buf.into_boxed_slice()).await
    }
}

#[async_trait]
impl<Fut> Output for Fut
where
    Fut: Future + Send,
    Fut::Output: Encode + TypeId,
{
    async fn produce<TR>(self, transport: &mut TR) -> io::Result<()>
    where
        TR: Transport + Send,
    {
        let mut buf = Vec::new();
        Encode::encode::<{ crate::DATABUF_CONFIG }>(&self.await, &mut buf)?;
        transport.send_unary_response(buf.into_boxed_slice()).await
    }
}

#[async_trait]
impl<G> Output for SSE<G>
where
    G: AsyncGenerator + Send,
{
    async fn produce<TR>(self, _: &mut TR) -> io::Result<()>
    where
        TR: Transport,
    {
        let mut gen = pin!(self.0);
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
                    break Ok(());
                }
            }
        }
    }
}
