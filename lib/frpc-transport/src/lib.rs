mod http;

pub use http::H2Transport;

type DynErr = Box<dyn std::error::Error + Send + Sync>;
