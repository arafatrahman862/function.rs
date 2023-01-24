#![allow(warnings)]
use std::sync::Arc;

async fn bind(config: quinn::ServerConfig, addr: std::net::SocketAddr) {
    let setver = quinn::Endpoint::server(config, addr).unwrap();

    while let Some(conn) = setver.accept().await {
        tokio::spawn(async {
            let conn = conn.await.unwrap();

            // let uni_conn = conn.clone();
            // tokio::spawn(async move { let _ = uni_conn.accept_uni().await; });

            while let Ok((writer, reader)) = conn.accept_bi().await {
                // tokio::spawn(future)
            }
        });
    }
}

pub fn server_config(
    certs: Vec<rustls::Certificate>,
    key: rustls::PrivateKey,
) -> quinn::ServerConfig {
    let mut tls_config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .unwrap();

    // tls_config.alpn_protocols = vec!["h3".into()];

    if cfg!(debug_assertions) {
        tls_config.key_log = Arc::new(rustls::KeyLogFile::new());
    }
    quinn::ServerConfig::with_crypto(Arc::new(tls_config))
}
