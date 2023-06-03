pub use h2x::Server;
use h2x::{http::StatusCode, *};
use std::future::Future;

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

pub async fn service<Func, State>(executor: Func, state: State, mut req: Request, mut res: Response)
where
    Func: for<'data, 'w> AsyncFnOnce<
        (State, u16, &'data [u8], &'w mut RpcTransport),
        Output = std::io::Result<()>,
    >,
{
    match req.headers.get("content-length") {
        Some(len) => {
            let Ok(Ok(len)) = len.to_str().map(str::parse::<u32>) else { return };
            if len > DEFAULT_MAX_UNARY_PAYLOAD_LEN {
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
                let mut writer = RpcTransport { inner: sender };
                if let Some(_err) = executor
                    .call_once((state, 0, &buf, &mut writer))
                    .await
                    .err()
                {
                    // ...
                }
            }
        }
        None => {
            // Stream ...
        }
    }
}

pub struct RpcTransport {
    inner: Responder,
}

#[async_trait::async_trait]
impl frpc::Transport for RpcTransport {
    async fn send_unary_response(&mut self, bytes: Box<[u8]>) -> std::io::Result<()> {
        let _ = self.inner.write_bytes(bytes.into(), true).await;
        //     .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err));
        Ok(())
    }
}
