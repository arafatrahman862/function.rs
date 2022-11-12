use std::{io::Result, net::TcpListener};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

macro_rules! do_else {
    [$item: expr $(, $do: expr)?] => {
        match $item {
            Ok(v) => v,
            Err(_) => {
                $(let _ = $do;)?
                return;
            },
        }
    };
}

pub async fn serve(listener: TcpListener, capacity: usize) -> Result<()> {
    listener.set_nonblocking(true)?;
    let listener = tokio::net::TcpListener::from_std(listener)?;
    loop {
        let (stream, _addr) = listener.accept().await?;
        tokio::spawn(async move {
            let mut stream = BufReader::with_capacity(capacity, stream);
            let data = do_else!(stream.fill_buf().await);

            if data.len() >= capacity {
                let _ = stream
                    .write_all(b"Contex Data larger than allowed or supported")
                    .await;

                return;
            }

            let prefix = b"RPC/TCP ";

            if let Some(_data) = data.strip_prefix(prefix) {
                do_else!(stream.write_all(prefix).await);
                stream.consume(prefix.len());
            } else {
                let _ = stream.write_all(b"Unsupported Protocol").await;
                return;
            }
        });
    }
}
