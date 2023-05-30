#![allow(warnings)]
use example::*;
use frpc_transport::H2Transport;

frpc::declare! {
    service Example {
        add = 1,
        print = 5
    }
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    frpc_codegen_client::init(Example);

    H2Transport::bind("127.0.0.1", "../cert/cert.pem", "../cert/key.pem")
        .await
        .unwrap()
        .serve(
            |_| Some(()),
            Example::execute,
            |_state, req, res| async move {
                let _ = res.write(format!("{}", req.uri)).await;
            },
        )
        .await;
}
