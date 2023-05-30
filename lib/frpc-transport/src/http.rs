use crate::DynErr;
use h2_plus::{
    http::{method::Method, HeaderValue, StatusCode},
    *,
};
use std::{future::Future, net::SocketAddr, path::Path};
use tokio::net::ToSocketAddrs;

pub trait AsyncFnOnce<Args> {
    type Output;
    type Future: Future<Output = Self::Output> + Send;
    fn call_once(self, _: Args) -> Self::Future;
}

impl<Func, Args, Fut, Ret> AsyncFnOnce<Args> for Func
where
    Func: frpc::fn_once::FnOnce<Args, Output = Fut>,
    Fut: Future<Output = Ret> + Send,
{
    type Output = Ret;
    type Future = Fut;

    fn call_once(self, args: Args) -> Self::Future {
        self.call_once(args)
    }
}

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
    ) -> Result<Self, DynErr> {
        Server::bind(addr, cert, key).await.map(Self::new)
    }

    pub fn new(server: Server) -> Self {
        Self {
            server,
            max_payload_len: DEFAULT_MAX_UNARY_PAYLOAD_LEN,
        }
    }

    pub async fn serve<State, Service, Stream>(
        mut self,
        on_accept: fn(SocketAddr) -> Option<State>,
        service: Service,
        on_stream: fn(State, Request, Response) -> Stream,
    ) where
        State: Clone + Send + 'static,
        Service: for<'data, 'w> AsyncFnOnce<
                (State, u16, &'data [u8], &'w mut RpcTransport),
                Output = std::io::Result<()>,
            >
            + Send
            + 'static
            + Copy,

        Stream: Future + Send + 'static,
    {
        loop {
            let Ok((mut conn, addr)) = self.server.accept().await else { continue };
            let Some(ctx) = on_accept(addr) else { continue };
            tokio::spawn(async move {
                while let Some(Ok((mut req, mut res))) = conn.accept().await {
                    let ctx = ctx.clone();
                    tokio::spawn(async move {
                        res.headers
                            .append("access-control-allow-origin", HeaderValue::from_static("*"));

                        if let (&Method::POST, "/rpc") = (&req.method, req.uri.path()) {
                            match req.headers.get("content-length") {
                                Some(len) => {
                                    let Ok(Ok(len)) = len.to_str().map(str::parse::<u32>) else { return };
                                    if len > self.max_payload_len {
                                        res.status = StatusCode::PAYLOAD_TOO_LARGE;
                                        let _ = res.send_headers();
                                        return;
                                    }
                                    let mut buf = Vec::with_capacity(len as usize);
                                    while let Some(Ok(bytes)) = req.data().await {
                                        buf.extend_from_slice(&bytes);
                                        if buf.len() > len as usize {
                                            res.status = StatusCode::BAD_REQUEST;
                                            let _ = res.send_headers();
                                            return;
                                        }
                                    }
                                    if let Ok(sender) = res.send_stream() {
                                        let mut writer = RpcTransport { sender };
                                        if let Some(_err) = service
                                            .call_once((ctx, 0, &buf, &mut writer))
                                            .await
                                            .err()
                                        {
                                            // ...
                                        };
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

pub struct RpcTransport {
    sender: Sender,
}

#[async_trait::async_trait]
impl frpc::Transport for RpcTransport {
    async fn send_unary_response(&mut self, bytes: Box<[u8]>) -> std::io::Result<()> {
        let _ = self.sender.write_bytes(bytes.into(), true).await;
        //     .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err));
        Ok(())
    }
}
