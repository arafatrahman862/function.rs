#![allow(dead_code)]

use frpc::procedure;

async fn foo() {}

async fn bar() {}

procedure! {
    rpc Prc {
        foo = 15,
        bar = 14,
    }
}
