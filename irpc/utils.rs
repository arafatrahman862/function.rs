use std::{future::Future, io::Result, net::TcpListener};
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncWrite, BufReader},
    net::TcpStream,
};

// type DynErr = Box<dyn std::error::Error + Send + Sync>;

pub async fn accept_connection<T>(
    listener: TcpListener,
    service: impl Fn(TcpStream) -> T,
) -> Result<()>
where
    T: Future + Send + 'static,
    T::Output: Send,
{
    listener.set_nonblocking(true)?;
    let listener = tokio::net::TcpListener::from_std(listener)?;
    loop {
        let (stream, _addr) = listener.accept().await?;
        tokio::spawn(service(stream));
    }
}

pub async fn get_transport(
    stream: TcpStream,
    capacity: usize,
) -> Result<impl AsyncRead + AsyncWrite> {
    let mut reader = BufReader::with_capacity(capacity, stream);
    let data = reader.fill_buf().await?;

    if let Some(_data) = data.strip_prefix(b"RPC/TCP ") {
    } else {

        // stream.write_all(src);
    }
    Ok(reader)
}
