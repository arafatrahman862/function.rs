use super::*;
use h2_plus::{
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

    pub async fn serve<Cx, Fut>(
        mut self,
        rpc: fn(u16, Vec<u8>),
        on_accept: fn(SocketAddr) -> Cx,
        mut on_stream: fn(Arc<Cx>, Request, Response) -> Fut,
    ) where
        Cx: Send + Sync + 'static,
        Fut: Future + Send + 'static,
    {
        loop {
            let Ok(((mut conn, addr))) = self.server.accept().await else { continue };
            let ctx = on_accept(addr);
            let ctx = Arc::new(ctx);
            tokio::spawn(async move {
                while let Some(Ok((mut req, mut res))) = conn.accept().await {
                    let ctx = Arc::clone(&ctx);
                    tokio::spawn(async move {
                        match (&req.method, req.uri.path()) {
                            (&Method::POST, "/rpc") => {
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
                                        rpc(0, buf);
                                    }
                                    None => {
                                        // Stream ...
                                    }
                                }
                            }
                            _ => {
                                on_stream(ctx, req, res).await;
                            }
                        }
                    });
                }
            });
        }
    }
}

async fn test() {
    let mut transport = H2Transport::bind("addr", "cert", "key").await;
    transport.max_payload_len = 545;
    transport.serve(
        |_, _| {},
        |_addr| {
            // ...
            ()
        },
        |ctx, req, mut res| async move {
            println!("{:?}", req.method);
            res.status = StatusCode::OK;
            let mut f = res.send_stream().unwrap();
            f.write("bytes").await;
            f.end_write("bytes");
        },
    );
}
