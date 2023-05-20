//!~
#![warn(missing_docs)]

#[doc(hidden)]
pub mod fn_once;
mod input;
mod output;
mod private;
mod state;

// mod service;
// pub use service::Service;

#[doc(hidden)]
#[cfg(debug_assertions)]
pub mod __private;

pub use databuf;

pub use frpc_macros::declare;
#[cfg(debug_assertions)]
pub use frpc_macros::Message;
#[cfg(not(debug_assertions))]
pub use frpc_macros::Noop as Message;

use input::Input;
pub use output::*;
pub use state::State;

#[doc(hidden)]
pub const DATABUF_CONFIG: u8 = databuf::config::num::LEB128 | databuf::config::len::BEU30;

#[doc(hidden)]
pub async fn run<'de, Args, Ret, State>(
    func: impl crate::fn_once::FnOnce<Args, Output = Ret>,
    state: State,
    reader: &mut &'de [u8],
    w: &mut (impl crate::output::Transport + Unpin + Send),
) -> std::io::Result<()>
where
    Args: Input<'de, State>,
    Ret: Output,
{
    let args = Args::decode(state, reader).unwrap();
    let output = func.call_once(args);
    Ret::produce(output, w).await
}
