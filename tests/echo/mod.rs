use frpc::declare;
use std::sync::Arc;
use std::{
    fmt::Debug,
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Debug, Default)]
pub struct Context {
    log_lvl: AtomicBool,
}

type State = frpc::State<Arc<Context>>;

async fn log(state: State, set: bool) {
    state.log_lvl.store(set, Ordering::Relaxed);
}

fn println(value: impl Debug) {
    println!("{:#?}", value);
}

macro_rules! def {
    [$($id: literal fn $name:ident -> $ty: ty)*] => {
        $(
            async fn $name(state: State, value: $ty) -> $ty {
                if state.log_lvl.load(Ordering::Acquire) { println(value); }
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
}
