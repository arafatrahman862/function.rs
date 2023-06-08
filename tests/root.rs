#![allow(warnings)]
mod validate;

use frpc_transport::http2::{
    http::{HeaderValue, Method},
    Server, TransportConfig,
};
use std::{fs, io::Result, ops::ControlFlow};

use validate::ValidateTest;

static RPC: TransportConfig = TransportConfig::new();

#[test]
fn codegen() {
    frpc_codegen_client::init(ValidateTest);
}

#[tokio::test]
async fn run() -> Result<()> {
    codegen();

    // let addr = "127.0.0.1:4433";
    // let cert = fs::read("example/cert/cert.pem")?;
    // let key = fs::read("example/cert/key.pem")?;

    // println!("Goto: https://{addr}");

    // Server::bind(addr, &mut &*cert, &mut &*key)
    //     .await
    //     .unwrap()
    //     .serve(
    //         |_| async { ControlFlow::Continue(Some(())) },
    //         |_conn, state, mut req, mut res| async move {
    //             res.headers
    //                 .append("access-control-allow-origin", HeaderValue::from_static("*"));

    //             match (&req.method, req.uri.path()) {
    //                 (&Method::POST, "/rpc") => RPC.service(Example::execute, state, req, res).await,
    //                 _ => {}
    //             }
    //         },
    //         |_| async {},
    //     )
    //     .await;

    Ok(())
}
