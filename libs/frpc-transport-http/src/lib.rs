#![allow(warnings)]
use frpc_transport_core::{BoxFuture, Transport};
use h2x::http::StatusCode;
pub use h2x::*;
use std::{
    fmt::Debug,
    future::poll_fn,
    io::Result,
    mem,
    task::{Context, Poll},
};

#[derive(Debug, Clone)]
pub struct Config {
    pub max_unary_payload_size: u32,
}
impl Config {
    pub const fn new() -> Self {
        Self {
            max_unary_payload_size: 128 * 1024,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Ctx<'req, 'res> {
    pub req: &'req mut Request,
    pub res: &'res mut Response,
}

impl<'req, 'res> Ctx<'req, 'res> {
    pub fn new(req: &'req mut Request, res: &'res mut Response) -> Self {
        Self { req, res }
    }

    pub async fn serve<'this, State>(
        &'this mut self,
        conf: &Config,
        state: State,
        // TODO: We should use a trait here.
        executor: impl for<'fut> FnOnce(
            State,
            u16,
            &'fut mut &[u8],
            &'fut mut RpcResponder<'this>,
        ) -> Option<BoxFuture<'fut, ()>>,
    ) -> std::result::Result<(), StatusCode> {
        match self.req.headers.get("content-length") {
            Some(len) => {
                let Ok(Ok(len)) = len.to_str().map(str::parse::<u32>) else { return Err(StatusCode::BAD_REQUEST) };
                if len > conf.max_unary_payload_size {
                    return Err(StatusCode::PAYLOAD_TOO_LARGE);
                }
                let mut buf = Vec::with_capacity(len as usize);
                while let Some(bytes) = self.req.data().await {
                    let Ok(bytes) = bytes else { return Err(StatusCode::PARTIAL_CONTENT); };
                    buf.extend_from_slice(&bytes);
                    if buf.len() > len as usize {
                        return Err(StatusCode::PARTIAL_CONTENT);
                    }
                }
                if buf.len() < 2 {
                    return Err(StatusCode::BAD_REQUEST);
                }
                let id = u16::from_le_bytes([buf[0], buf[1]]);
                let data = &buf[2..];

                let mut transport = RpcResponder(self.res);
                let mut cursor = data;
                let Some(fut) = executor(state, id, &mut cursor, &mut transport) else { return Err(StatusCode::NOT_FOUND) };
                fut.await;
            }
            None => {
                // Uni-Stream, Bi-Stream
                return Err(StatusCode::NOT_IMPLEMENTED);
            }
        }
        Ok(())
    }
}

pub struct RpcResponder<'a>(&'a mut Response);

impl Transport for RpcResponder<'_> {
    fn unary_sync<'this, 'fut, CB>(&'this mut self, cb: CB) -> BoxFuture<'fut, ()>
    where
        'this: 'fut,
        Self: 'fut,
        CB: for<'buf> FnOnce(&'buf mut Vec<u8>) -> Result<()> + Send + 'fut,
    {
        let mut cb = Some(cb);
        self.unary(move |_, buf| {
            Poll::Ready(match cb.take() {
                Some(cb) => cb(buf),
                None => unreachable!(),
            })
        })
    }

    fn unary<'this, 'fut, P>(&'this mut self, mut poll: P) -> BoxFuture<'fut, ()>
    where
        'this: 'fut,
        Self: 'fut,
        P: Send + 'fut,
        P: for<'cx, 'w, 'buf> FnMut(&'cx mut Context<'w>, &'buf mut Vec<u8>) -> Poll<Result<()>>,
    {
        Box::pin(async move {
            let mut response = http::Response::new(());
            *response.headers_mut() = mem::replace(&mut self.0.headers, Default::default());
            let mut buf = vec![];
            match poll_fn(|cx| poll(cx, &mut buf)).await {
                Ok(_) => {
                    let is_empty = buf.is_empty();
                    if let Ok(stream) = self.0.sender.send_response(response, is_empty) {
                        if !is_empty {
                            h2x::Responder { inner: stream }
                                .write_bytes(buf.into(), true)
                                .await;
                        }
                    }
                }
                Err(_parse_err) => {
                    // dbg!(_parse_err);
                    *response.status_mut() = StatusCode::NOT_ACCEPTABLE;
                    let _ = self.0.sender.send_response(response, true);
                }
            }
        })
    }

    fn server_stream<'this, 'fut, P>(&'this mut self, mut poll: P) -> BoxFuture<'fut, ()>
    where
        'this: 'fut,
        Self: 'fut,
        P: Send + 'fut,
        P: for<'cx, 'w, 'buf> FnMut(&'cx mut Context<'w>, &'buf mut Vec<u8>) -> Poll<Result<bool>>,
    {
        Box::pin(async move {
            let mut response = http::Response::new(());
            *response.headers_mut() = mem::replace(&mut self.0.headers, Default::default());
            let mut buf = vec![0; 4];
            loop {
                match poll_fn(|cx| poll(cx, &mut buf)).await {
                    Ok(done) => match done {
                        true => {}
                        false => {}
                    },
                    Err(_parse_err) => {
                        // dbg!(_parse_err);
                        *response.status_mut() = StatusCode::NOT_ACCEPTABLE;
                        let _ = self.0.sender.send_response(response, true);
                        break;
                    }
                }
            }
        })
    }
}
