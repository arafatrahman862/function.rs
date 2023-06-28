//! cargo test --test rpc
mod echo;
mod validate;

use frpc_transport::http2::{http::HeaderValue, Config, Ctx, Server};
use std::{
    collections::HashSet,
    fs,
    io::Result,
    ops::ControlFlow,
    process::{Command, Output, Stdio},
    sync::Arc,
};
use tokio::task;

use echo::{Context, EchoTest};
use validate::ValidateTest;

static CONF: Config = Config::new();

#[tokio::main]
async fn main() -> Result<()> {
    let args: HashSet<_> = std::env::args().skip(1).collect();

    let time = std::time::Instant::now();
    frpc_codegen_client::init(EchoTest);
    frpc_codegen_client::init(ValidateTest);
    println!("Codegen finished in {:?}\n", time.elapsed());

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
            |_conn, state, mut req, mut res| async move {
                res.headers
                    .append("access-control-allow-origin", HeaderValue::from_static("*"));

                let mut ctx = Ctx::new(&mut req, &mut res);
                let _ = match ctx.req.uri.path() {
                    "/rpc/validate" => ctx.serve(&CONF, (), ValidateTest::execute).await,
                    "/rpc/echo" => ctx.serve(&CONF, state, EchoTest::execute).await,
                    _ => return,
                };
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

fn run_clients() -> Result<()> {
    run_js("./tests/echo/mod.ts")?;
    run_js("./tests/validate/mod.ts")?;
    Ok(())
}

fn run_js(path: &str) -> Result<Output> {
    Command::new("deno")
        .args([
            "run",
            "--allow-net=localhost",
            "--unsafely-ignore-certificate-errors=localhost",
            path,
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
}
