mod http2;
pub use http2::*;

type DynErr = Box<dyn std::error::Error + Send + Sync>;
