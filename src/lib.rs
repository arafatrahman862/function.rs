#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod input;
mod output;
mod private;
mod state;

// mod service;
// pub use service::Service;

#[doc(hidden)]
#[cfg(debug_assertions)]
pub mod __private;

use async_gen::GeneratorState;
use databuf::Encode;
pub use frpc_macros::{declare, Message};
pub use output::*;
pub use state::State;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

#[doc(hidden)]
pub const DATABUF_CONFIG: u8 = databuf::config::num::LEB128 | databuf::config::len::BEU30;

#[doc(hidden)]
pub async fn run<'de, Args, Ret, State>(
    func: impl std_lib::FnOnce<Args, Output = Ret>,
    state: State,
    reader: &mut &'de [u8],
    w: &mut (impl crate::output::Transport + Unpin + Send),
) -> databuf::Result<()>
where
    Args: input::Input<'de, State>,
    Ret: Output,
{
    let args = Args::decode(state, reader)?;
    let output = func.call_once(args);
    Ret::produce(output, w).await?;
    Ok(())
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
    G::Yield: Encode + frpc_message::TypeId,
    G::Return: Encode + frpc_message::TypeId,
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
