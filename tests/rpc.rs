//! Run: cargo test --package frpc --test root -- --nocapture

mod echo;
mod validate;

use frpc_transport::http2::{
    http::{HeaderValue, Method},
    Server, TransportConfig,
};
use std::{
    collections::HashSet,
    fs,
    io::Result,
    ops::ControlFlow,
    process::{Command, Stdio},
    sync::Arc,
};
use tokio::task;

// ------------------------------------------------------------

use echo::{Context, EchoTest};
use validate::ValidateTest;

static RPC: TransportConfig = TransportConfig::new();

fn codegen() {
    let time = std::time::Instant::now();
    frpc_codegen_client::init(EchoTest);
    frpc_codegen_client::init(ValidateTest);
    println!("Codegen finished in {:?}\n", time.elapsed());
}

fn run_clients() -> Result<()> {
    Command::new("deno")
        .args([
            "run",
            "--allow-net=localhost",
            "--unsafely-ignore-certificate-errors=localhost",
            "./tests/echo/mod.ts",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: HashSet<_> = std::env::args().skip(1).collect();

    codegen();
    if args.contains("codegen") {
        return Ok(());
    }

    let addr = "127.0.0.1:4433";
    let cert = fs::read(".vscode/cert.pem")?;
    let key = fs::read(".vscode/key.pem")?;

    let (server, recv_close_signal) = Server::bind(addr, &mut &*cert, &mut &*key)
        .await
        .unwrap()
        .serve_with_graceful_shutdown(
            |_| async { ControlFlow::Continue(Some(Arc::new(Context::default()))) },
            |_conn, state, req, mut res| async move {
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

    if args.contains("serve") {
        println!("Server runing...");
        println!("Goto: https://{addr}");
        server.await;
    } else {
        tokio::select! {
            output = task::spawn_blocking(run_clients) => output??,
            _ = server => {},
        }
    }
    Ok(recv_close_signal.await)
}
