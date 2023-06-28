use h2x::http::StatusCode;
pub use h2x::*;
use std::{
    fmt::Debug,
    io,
    task::{Context, Poll},
};

pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

#[derive(Debug, Clone)]
pub struct TransportConfig {
    pub max_unary_payload_size: u32,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self::new()
    }
}
impl TransportConfig {
    pub const fn new() -> Self {
        Self {
            max_unary_payload_size: 128 * 1024,
        }
    }

    pub async fn service<State>(
        &self,
        executor: impl for<'fut, 'data> FnOnce(
            State,
            u16,
            &'fut mut &'data [u8],
            // &'fut mut RpcResponder,
            RpcResponder<'fut>,
        ) -> Option<BoxFuture<'fut, ()>>,
        state: State,
        req: &mut Request,
        res: &mut Response,
    ) {
        match req.headers.get("content-length") {
            Some(len) => {
                let Ok(Ok(len)) = len.to_str().map(str::parse::<u32>) else { return };
                if len > self.max_unary_payload_size {
                    res.status = StatusCode::PAYLOAD_TOO_LARGE;
                    // let _ = res.send_headers();
                    return;
                }
                let mut buf = Vec::with_capacity(len as usize);
                while let Some(Ok(bytes)) = req.data().await {
                    buf.extend_from_slice(&bytes);
                    if buf.len() > len as usize {
                        res.status = StatusCode::BAD_REQUEST;
                        // let _ = res.send_headers();
                        return;
                    }
                }
                // let Ok(responder) = res.send_stream() else { return };
                if buf.len() < 2 {
                    return;
                }
                let id = u16::from_le_bytes([buf[0], buf[1]]);
                let data = &buf[2..];

                let mut writer = RpcResponder(res);
                let mut reader = data;
                if let Some(fut) = executor(state, id, &mut reader, writer) {
                    fut.await;
                };
            }
            None => {
                // Uni-Stream, Bi-Stream
            }
        }
    }
}

pub struct RpcResponder<'a>(&'a mut Response);

#[async_trait::async_trait]
impl frpc::Transport for RpcResponder<'_> {
    // async fn send_unary_response(&mut self, bytes: Box<[u8]>) {
    //     if bytes.is_empty() {
    //         let _ = self.0.inner.send_data(bytes::Bytes::new(), true);
    //     } else {
    //         let _ = self.0.write_bytes(bytes.into(), true).await;
    //     }
    // }

    // async fn unary_response_sync<W: std::io::Write + 'life1>(
    //     &mut self,
    //     func: impl FnOnce(&mut W) -> std::io::Result<()> + Send,
    // ) {
    //     let mut buf = vec![];
    //     // func(&mut buf);
    //     todo!()
    // }

    async fn server_stream(
        &mut self,
        mut poll: impl for<'cx, 'waker, 'buf> FnMut(
                &'cx mut Context<'waker>,
                &'buf mut Vec<u8>,
            ) -> Poll<io::Result<bool>>
            + Send,
    ) {
    }

    async fn unary(
        &mut self,
        mut poll: impl for<'cx, 'waker, 'buf> FnMut(
                &'cx mut Context<'waker>,
                &'buf mut Vec<u8>,
            ) -> Poll<io::Result<()>>
            + Send,
    ) {
    }

    async fn unary_sync(
        &mut self,
        cb: impl for<'buf> FnOnce(&'buf mut Vec<u8>) -> io::Result<()> + Send,
    ) {
        // let mut buf = vec![];
        // match cb(&mut buf) {
        //     Ok(_) => {}
        //     Err(err) => {}
        // }
    }
}
