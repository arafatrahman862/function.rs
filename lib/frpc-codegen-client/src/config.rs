use std::{env, path::PathBuf};

use serde::Deserialize;

pub mod typescript {
    use super::*;

    #[derive(Debug, Deserialize, Default)]
    pub struct Config {
        #[serde(default)]
        #[serde(rename = "import-with-extension")]
        pub import_with_extension: bool,
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "out_dir")]
    #[serde(rename = "out-dir")]
    pub out_dir: PathBuf,
    pub typescript: Option<typescript::Config>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            out_dir: out_dir(),
            typescript: Default::default(),
        }
    }
}

fn out_dir() -> PathBuf {
    if let Ok(dir) = env::var("CARGO_TARGET_DIR") {
        return dir.into();
    }
    if let Ok(cwd) = env::current_dir() {
        return cwd;
    }
    "/".into()
}
