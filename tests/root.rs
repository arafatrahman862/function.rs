#![allow(warnings)]
mod echo;
mod validate;

use frpc_transport::http2::{
    http::{HeaderValue, Method},
    Server, TransportConfig,
};
use std::{fs, io::Result, ops::ControlFlow, sync::Arc};

use echo::{Context, EchoTest};
use validate::ValidateTest;

static RPC: TransportConfig = TransportConfig::new();

#[test]
fn codegen() {
    let time = std::time::Instant::now();
    std::thread::scope(|thread| {
        thread.spawn(|| frpc_codegen_client::init(EchoTest));
        thread.spawn(|| frpc_codegen_client::init(ValidateTest));
    });
    println!("Codegen finished in {:?}", time.elapsed());
}

#[tokio::test]
async fn run() -> Result<()> {
    codegen();

    let addr = "127.0.0.1:4433";
    let cert = fs::read(".vscode/cert.pem")?;
    let key = fs::read(".vscode/key.pem")?;

    println!("Goto: https://{addr}");

    let (server, close_signal) = Server::bind(addr, &mut &*cert, &mut &*key)
        .await
        .unwrap()
        .serve_with_graceful_shutdown(
            |_| async { ControlFlow::Continue(Some(Arc::new(Context::default()))) },
            |_conn, state, mut req, mut res| async move {
                res.headers
                    .append("access-control-allow-origin", HeaderValue::from_static("*"));

                match (&req.method, req.uri.path()) {
                    (&Method::POST, "/rpc/validate") => {
                        RPC.service(ValidateTest::execute, (), req, res).await
                    }
                    (&Method::POST, "/rpc/echo") => {
                        RPC.service(EchoTest::execute, state, req, res).await
                    }
                    _ => {}
                }
            },
            |_| async {},
        );

    // tokio::select! {
    //     _ = server => {}
    // }
    server.await;
    close_signal.await;
    Ok(())
}
