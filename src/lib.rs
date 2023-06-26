#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]

mod input;
mod output;
mod output_type;
mod state;
// mod service;

#[doc(hidden)]
#[cfg(debug_assertions)]
pub mod __private;
pub use async_gen;

pub use frpc_macros::{declare, Message};
pub use output::*;
pub use state::State;
// pub use service::Service;

use async_gen::GeneratorState;
use async_trait::async_trait;
use databuf::Encode;
use frpc_message::TypeId;

use std::{
    future::{poll_fn, Future},
    io,
    pin::{pin, Pin},
    task::{Context, Poll},
};

#[doc(hidden)]
pub const DATABUF_CONFIG: u8 = databuf::config::num::LEB128 | databuf::config::len::BEU30;

#[doc(hidden)]
pub async fn run<'de, State, Args, Ret>(
    func: impl std_lib::FnOnce<Args, Output = Ret>,
    state: State,
    reader: &mut &'de [u8],
    transport: &mut (impl Transport + Send),
) where
    Args: input::Input<'de, State>,
    Ret: Output,
{
    let args = Args::decode(state, reader);
    todo!()
    // let output = func.call_once(args);
    // Ret::produce(output, transport).await;
}

pub struct SSE<G>(pub G);
pub struct Return<T>(pub T);

#[macro_export]
macro_rules! sse {
    ($($tt:tt)*) => {
        $crate::SSE($crate::async_gen::__private::gen_inner!([$crate::async_gen] $($tt)*))
    }
}

#[allow(missing_docs)]
pub trait AsyncGenerator {
    type Yield: Encode + frpc_message::TypeId;
    type Return: Encode + frpc_message::TypeId;

    fn poll_resume(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<GeneratorState<Self::Yield, Self::Return>>;
}

impl<G> AsyncGenerator for G
where
    G: async_gen::AsyncGenerator,
    G::Yield: Encode + TypeId,
    G::Return: Encode + TypeId,
{
    type Yield = G::Yield;
    type Return = G::Return;

    fn poll_resume(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<GeneratorState<Self::Yield, Self::Return>> {
        G::poll_resume(self, cx)
    }
}
