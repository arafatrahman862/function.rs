use std::env;
use std::path::PathBuf;

pub(crate) fn manifest_dir() -> PathBuf {
    env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .or_else(|_| env::current_dir())
        .unwrap_or("./".into())
}

pub(crate) fn pkg_name() -> String {
    env::var("CARGO_PKG_NAME").unwrap_or("frpc.stub".into())
}
