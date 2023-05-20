#![allow(warnings)]
use frpc::declare;

async fn foo() {}
async fn bar() {}

declare! {
    service Prc {
        foo = 15,
        bar = 14,
    }
}

fn main() {}
