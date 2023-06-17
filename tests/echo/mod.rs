use databuf::Decode;
use frpc::declare;
use frpc_macros::Message;
use std::sync::Arc;
use std::{
    fmt::Debug,
    sync::atomic::{AtomicBool, Ordering},
};

fn println(value: &impl Debug) {
    println!("{:#?}", value);
}

macro_rules! def {
    [$($id: literal fn $name:ident -> $ty: ty)*] => {
        $(
            async fn $name(state: State, value: $ty) -> $ty {
                if state.log_lvl.load(Ordering::Acquire) { println(&value); }
                value
            }
        )*
        declare! {
            type State = Arc<Context>;
            pub service EchoTest {
                log = 0,
                $($name = $id),*
            }
        }
    };
}

#[derive(Debug, Default)]
pub struct Context {
    log_lvl: AtomicBool,
}

type State = frpc::State<Arc<Context>>;

#[derive(Message, Decode, Debug)]
enum Log {
    Disable,
    Enable,
}

async fn log(state: State, log: Log) {
    match log {
        Log::Enable => state.log_lvl.store(true, Ordering::Relaxed),
        Log::Disable => state.log_lvl.store(false, Ordering::Relaxed),
    }
}

def! {
    // Number
    1 fn echo_u8 -> u8
    2 fn echo_u16 -> u16
    3 fn echo_u32 -> u32
    4 fn echo_u64 -> u64
    5 fn echo_u128 -> u128
    6 fn echo_usize -> usize

    // Neg Number
    7 fn echo_i8 -> i8
    8 fn echo_i16 -> i16
    9 fn echo_i32 -> i32
    10 fn echo_i64 -> i64
    11 fn echo_i128 -> i128
    12 fn echo_isize -> isize

    // Flote Number
    13 fn echo_f32 -> f32
    14 fn echo_f64 -> f64

    // other
    15 fn echo_option -> Option<Option<&str>>
    16 fn echo_result -> Result<String, String>

    // -----------------

    17 fn echo_str -> &str
    18 fn echo_bool -> bool
    19 fn echo_bytes -> Vec<u8>
}
