#![allow(dead_code)]

async fn add(a: i32, b: i32) -> i32 {
    a + b
}
async fn print(_msg: String) {}

frpc::declare! {
    service Example {
        add = 1,
        print = 5
    }
}

fn main() {
    frpc_codegen_client::init(Example);
}
