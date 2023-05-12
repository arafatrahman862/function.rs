use super::*;
use h2_plus::{
    bytes::Bytes,
    http::{method::Method, StatusCode},
    *,
};
use std::{future::Future, net::SocketAddr, path::Path, sync::Arc};
use tokio::net::ToSocketAddrs;

const DEFAULT_MAX_UNARY_PAYLOAD_LEN: u32 = 128 * 1024;

pub struct H2Transport {
    pub server: Server,
    pub max_payload_len: u32,
}

impl H2Transport {
    pub async fn bind(
        addr: impl ToSocketAddrs,
        cert: impl AsRef<Path>,
        key: impl AsRef<Path>,
    ) -> Self {
        Self::new(Server::bind(addr, cert, key).await.unwrap())
    }

    pub fn new(server: Server) -> Self {
        Self {
            server,
            max_payload_len: DEFAULT_MAX_UNARY_PAYLOAD_LEN,
        }
    }

    pub async fn serve<Ctx, Rpc, Stream>(
        mut self,
        rpc: fn(Ctx, u16, Vec<u8>, Writer) -> Rpc,
        on_accept: fn(SocketAddr) -> Option<Ctx>,
        on_stream: fn(Ctx, Request, Response) -> Stream,
    ) where
        Ctx: Clone + Send + 'static,
        Rpc: Future<Output = std::io::Result<()>> + Send + 'static,
        Stream: Future + Send + 'static,
    {
        loop {
            let Ok(((mut conn, addr))) = self.server.accept().await else { continue };
            let Some(ctx) = on_accept(addr) else { continue };
            tokio::spawn(async move {
                while let Some(Ok((mut req, mut res))) = conn.accept().await {
                    let ctx = ctx.clone();
                    tokio::spawn(async move {
                        if let (&Method::POST, "/rpc") = (&req.method, req.uri.path()) {
                            match req.headers.get("content-length") {
                                Some(len) => {
                                    let Ok(Ok(len)) = len.to_str().map(str::parse::<u32>) else { return };
                                    if len > self.max_payload_len {
                                        res.status = StatusCode::PAYLOAD_TOO_LARGE;
                                        res.send_headers();
                                        return;
                                    }
                                    let mut buf = Vec::with_capacity(len as usize);
                                    while let Some(Ok(bytes)) = req.data().await {
                                        buf.extend_from_slice(&bytes);
                                        if buf.len() > len as usize {
                                            res.status = StatusCode::BAD_REQUEST;
                                            res.send_headers();
                                            return;
                                        }
                                    }
                                    if let Ok(sender) = res.send_stream() {
                                        let mut writer = Writer { sender };
                                        rpc(ctx, 0, buf, writer).await;
                                    }
                                }
                                None => {
                                    // Stream ...
                                }
                            }
                        } else {
                            on_stream(ctx, req, res).await;
                        }
                    });
                }
            });
        }
    }
}

struct Writer {
    sender: Sender,
}

#[async_trait::async_trait]
impl frpc::output::AsyncWriter for Writer {
    async fn write_boxed_slice(&mut self, buf: Box<[u8]>) -> std::io::Result<()> {
        self.sender
            .write_bytes(buf.into(), false)
            .await
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
    }

    fn end(&mut self) {
        self.sender.inner.send_data(Bytes::new(), true);
    }

    async fn end_write(&mut self, bytes: Box<[u8]>) -> std::io::Result<()> {
        self.sender.write_bytes(bytes.into(), true).await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    async fn awd() {}
    async fn awds() {}

    frpc::procedure! {
        awd = 1
        awds = 2
    }

    async fn test_name() {
        let mut transport: H2Transport = H2Transport::bind("addr", "cert", "key").await;
        transport
            .serve(
                |ctx, id, data, mut writer| async move {
                    procedure::execute(ctx, id, data, &mut writer).await
                },
                |_addr| Some(()),
                move |ctx, req, mut res| async move {
                    println!("{:?}", req.method);
                },
            )
            .await;
    }
}
