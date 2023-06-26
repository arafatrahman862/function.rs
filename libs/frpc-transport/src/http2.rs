use std::fmt::Debug;

use h2x::http::StatusCode;
pub use h2x::*;
use std_lib::AsyncFnOnce;

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

    pub async fn service<Func, State>(
        &self,
        executor: Func,
        state: State,
        mut req: Request,
        mut res: Response,
    ) where
        Func: for<'data, 'w> AsyncFnOnce<(State, u16, &'data [u8], &'w mut RpcResponder)>,
    {
        match req.headers.get("content-length") {
            Some(len) => {
                let Ok(Ok(len)) = len.to_str().map(str::parse::<u32>) else { return };
                if len > self.max_unary_payload_size {
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
                let Some(id) = buf.get(..2) else { return };
                let id = u16::from_le_bytes(id.try_into().unwrap());
                let data = &buf[2..];

                if let Ok(sender) = res.send_stream() {
                    let mut writer = RpcResponder(sender);
                    executor.call_once((state, id, data, &mut writer)).await;
                }
            }
            None => {
                // Stream ...
            }
        }
    }
}

pub struct RpcResponder(Responder);

#[async_trait::async_trait]
impl frpc::Transport for RpcResponder {
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
}
