use async_trait::async_trait;
use std::{future::Future, io};

#[async_trait]
pub trait AsyncWriter: Sized {
    async fn write_boxed_slice(&mut self, _: Box<[u8]>) -> std::io::Result<()>;
    async fn end_write(&mut self, _: Box<[u8]>) -> std::io::Result<()>;
    fn end(&mut self) {}
}

#[async_trait]
impl<T> AsyncWriter for T
where
    T: std::io::Write + Send,
{
    async fn write_boxed_slice(&mut self, buf: Box<[u8]>) -> io::Result<()> {
        self.write_all(&buf)
    }
    async fn end_write(&mut self, buf: Box<[u8]>) -> std::io::Result<()> {
        self.write_all(&buf)
    }
}

#[async_trait]
pub trait Output: crate::private::Sealed {
    async fn send_output<W>(self, _: &mut W) -> io::Result<()>
    where
        W: AsyncWriter + Unpin + Send;
}

#[async_trait]
impl<Fut, T> Output for Fut
where
    Fut: Future<Output = T> + Send,
    T: databuf::Encode,
{
    async fn send_output<W>(self, writer: &mut W) -> io::Result<()>
    where
        W: AsyncWriter + Unpin + Send,
    {
        let mut buf = Vec::new();
        T::encode::<{ crate::DATABUF_CONFIG }>(&self.await, &mut buf)?;
        writer.end_write(buf.into_boxed_slice()).await
    }
}
