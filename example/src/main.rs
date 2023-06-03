#![allow(warnings)]
use example::*;
use frpc_transport::{
    h2_plus::http::{HeaderValue, Method},
    service, H2Transport,
};

frpc::declare! {
    service Example {
        add = 1,
        print = 5
    }
}

trait Foo {
    type State;
    fn execute<W>(&self, state: Self::State, id: u16, data: &[u8], w: &mut W)
    where
        W: ::frpc::Transport + Unpin + Send;
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    frpc_codegen_client::init(Example);

    H2Transport::bind("127.0.0.1", "../cert/cert.pem", "../cert/key.pem")
        .await
        .unwrap()
        .serve(
            move |_| Some(()),
            |state, req, mut res| async move {
                res.headers
                    .append("access-control-allow-origin", HeaderValue::from_static("*"));

                match (req.method.clone(), req.uri.path()) {
                    (Method::POST, "/rpc") => {
                        service(Example::execute, state, req, res).await;
                    }
                    (method, url) => {
                        let _ = res.write(format!("Unknown: {method} {url}")).await;
                    }
                }
            },
        )
        .await;
}
