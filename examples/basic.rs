#![allow(warnings)]
use frpc::declare;

async fn foo() {}
async fn bar() {}

declare! {
    service Bar {
        foo = 15,
        bar = 14,
    }
}

fn main() {}
