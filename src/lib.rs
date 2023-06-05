//!~
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
    func: impl std_lib::FnOnce<Args, Output = Ret>,
    state: State,
    reader: &mut &'de [u8],
    w: &mut (impl crate::output::Transport + Unpin + Send),
) -> databuf::Result<()>
where
    Args: Input<'de, State>,
    Ret: Output,
{
    let args = Args::decode(state, reader)?;
    let output = func.call_once(args);
    Ret::produce(output, w).await?;
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     #![allow(warnings)]
//     use databuf::{Decode, Encode};

//     use super::*;

//     #[test]
//     fn test_name() {
//         let mut data = vec![];
//         10_i32.encode::<{ DATABUF_CONFIG }>(&mut data);
//         20_i32.encode::<{ DATABUF_CONFIG }>(&mut data);
//         println!("{:?}", data);

//         let data = &mut &*data;

//         let f: (i32, i32) = Input::decode((), data).unwrap();
//         println!("{:?}", f);
//         // println!("{:?}", i32::decode::<{ DATABUF_CONFIG }>(data));
//         // println!("{:?}", i32::decode::<{ DATABUF_CONFIG }>(data));
//         // println!("{:?}", i32::decode::<{ DATABUF_CONFIG }>(data));
//     }
// }
