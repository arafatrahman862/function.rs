#![allow(warnings)]
use frpc_transport::http2::{
    http::{HeaderValue, Method},
    Server, TransportConfig,
};
use std::{fs, io::Result, ops::ControlFlow};

use example::Example;

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    frpc_codegen_client::init(Example);

    let addr = "127.0.0.1:4433";
    let cert = fs::read("example/cert/cert.pem")?;
    let key = fs::read("example/cert/key.pem")?;

    println!("Goto: https://{addr}");

    Server::bind(addr, &mut &*cert, &mut &*key)
        .await
        .unwrap()
        .serve(
            |_| async { ControlFlow::Continue(Some(())) },
            |_conn, state, mut req, mut res| async move {
                res.headers
                    .append("access-control-allow-origin", HeaderValue::from_static("*"));

                let rpc = TransportConfig::new();

                println!("{:#?}", req);

                match (&req.method, req.uri.path()) {
                    (&Method::POST, "/rpc") => {
                        rpc.service(Example::execute, state, req, res).await;
                    }
                    _ => {
                        let mut stream = res.send_stream().unwrap();
                        stream.write(format!("{req:#?}\n\n")).await;

                        while let Some(Ok(bytes)) = req.data().await {
                            stream.write(bytes).await;
                        }
                        stream.end();
                    }
                }
            },
            |_| async {},
        )
        .await;

    Ok(())
}
