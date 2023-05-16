#![allow(dead_code)]
//! Prototype API

use super::*;
use crate::{
    input::Input,
    output::{AsyncWriter, Output},
};
use std::{collections::HashMap, future::Future, marker::PhantomData, pin::Pin};

type RpcFutute<'a, Output = std::io::Result<()>> =
    Pin<Box<dyn Future<Output = Output> + Send + 'a>>;

trait RPC<State, AsyncWriter> {
    // type Output;
    // type Future<'a>: Future<Output = Self::Output> + Send + 'a;
    fn call<'a>(&self, state: State, data: Box<[u8]>, w: &'a mut AsyncWriter) -> RpcFutute<'a>;
}

struct Handler<Func, Args> {
    func: Func,
    _phantom_args: PhantomData<Args>,
}

impl<State, Writer, Func, Args, Ret> RPC<State, Writer> for Handler<Func, Args>
where
    Func: fn_once::FnOnce<Args, Output = Ret> + Clone,
    Args: for<'de> Input<'de, State>,
    Ret: Output + 'static,
    Writer: AsyncWriter + Unpin + Send,
{
    // type Output = Result<()>;
    // type Future<'a> = RpcFutute<'a>;
    fn call<'f>(&self, state: State, data: Box<[u8]>, w: &'f mut Writer) -> RpcFutute<'f> {
        let mut reader = &*data;
        let args = Args::decode(state, &mut reader).unwrap();
        let output = self.func.clone().call_once(args);
        Output::send_output(output, w)
    }
}

pub struct Rpc<Writer, State = ()> {
    handlers: HashMap<u16, Box<dyn RPC<State, Writer>>>,
}

impl<Writer, State> Rpc<Writer, State>
where
    Writer: AsyncWriter + Unpin + Send,
{
    pub fn new() -> Self {
        Self {
            handlers: Default::default(),
        }
    }

    pub fn export<Func, Args, Ret>(mut self, id: u16, _name: &str, func: Func) -> Self
    where
        Func: fn_once::FnOnce<Args, Output = Ret> + Clone + 'static,
        Args: for<'de> Input<'de, State> + 'static,
        Ret: Future + Send + 'static,
        Ret::Output: databuf::Encode,
    {
        self.handlers.insert(
            id,
            Box::new(Handler {
                func,
                _phantom_args: PhantomData::<Args>,
            }),
        );
        self
    }

    pub fn call<'a>(
        &self,
        state: State,
        id: u16,
        data: Box<[u8]>,
        w: &'a mut Writer,
    ) -> Option<RpcFutute<'a>> {
        Some(self.handlers.get(&id)?.call(state, data, w))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn foo(_: State<()>) {}
    async fn get_num() -> &'static str {
        "42"
    }

    #[tokio::test]
    async fn run() {
        let rpc = Rpc::new()
            .export(1, "foo", foo)
            .export(2, "get_num", get_num);

        let mut output = vec![];
        let _ = rpc.call((), 2, Box::new([]), &mut output).unwrap().await;
        assert_eq!(output, [2, b'4', b'2']);
    }
}
