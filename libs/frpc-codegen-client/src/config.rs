use std::{env, path::PathBuf};

use serde::Deserialize;

pub mod typescript {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct Config {
        #[serde(default = "out_dir")]
        #[serde(rename = "out-dir")]
        pub out_dir: PathBuf,
        #[serde(default)]
        #[serde(rename = "preserve-import-extension")]
        pub preserve_import_extension: bool,
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                out_dir: out_dir(),
                preserve_import_extension: Default::default(),
            }
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub typescript: Option<typescript::Config>,
}

fn out_dir() -> PathBuf {
    if let Ok(dir) = env::var("CARGO_TARGET_DIR") {
        return dir.into();
    }
    if let Ok(cwd) = env::current_dir() {
        return cwd;
    }
    manifest_dir()
}

pub(crate) fn manifest_dir() -> PathBuf {
    if let Ok(dir) = env::var("CARGO_MANIFEST_DIR") {
        return dir.into();
    }
    "./".into()
}
