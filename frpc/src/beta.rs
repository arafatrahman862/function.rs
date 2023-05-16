#![allow(warnings)]
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

impl<State, Func, Args, Ret, Writer> RPC<State, Writer> for Handler<Func, Args>
where
    Func: fn_once::FnOnce<Args, Output = Ret> + Copy,
    Args: for<'de> Input<'de, State>,
    Ret: Output + 'static,
    Writer: AsyncWriter + Unpin + Send,
{
    // type Output = Result<()>;
    // type Future<'a> = RpcFutute<'a>;
    fn call<'a>(&self, state: State, data: Box<[u8]>, w: &'a mut Writer) -> RpcFutute<'a> {
        let mut reader = &*data;
        let args = Args::decode(state, &mut reader).unwrap();
        let output = self.func.call_once(args);
        Output::send_output(output, w)
    }
}

struct Route<CreateStare, State, Writer> {
    pub create_state: CreateStare,
    pub handlers: HashMap<u16, Box<dyn RPC<State, Writer>>>,
}

impl<CreateStare, State, Writer> Route<CreateStare, State, Writer>
where
    CreateStare: FnOnce() -> State,
    Writer: AsyncWriter + Unpin + Send,
{
    fn new(ctx: CreateStare) -> Self {
        Self {
            create_state: ctx,
            handlers: Default::default(),
        }
    }

    fn export<Func, Args, Ret>(mut self, id: u16, name: &str, func: Func) -> Self
    where
        Func: fn_once::FnOnce<Args, Output = Ret> + Copy + 'static,
        Args: for<'de> input::Input<'de, State> + 'static,
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

    fn call<'a>(
        &self,
        state: State,
        id: u16,
        data: Box<[u8]>,
        w: &'a mut Writer,
    ) -> Option<RpcFutute<'a>> {
        Some(self.handlers.get(&id)?.call(state, data, w))
    }
}

#[derive(Clone)]
struct A;

async fn a(a: State<A>, data: Box<[u8]>) {}

#[test]
fn test_name() {
    let route: Route<_, _, Vec<u8>> = Route::new(|| A)
        .export(0, "name", a)
        .export(0, "name", a)
        .export(0, "name", a);
}
