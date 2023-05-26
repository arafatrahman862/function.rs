use std::{fs, path::Path};

use frpc_codegen::CodeGen;
pub use frpc_message::TypeDef;
pub mod config;
pub use config::Config;

pub fn init(_td: impl Into<TypeDef>) {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let config_file_path = manifest_dir.join("codegen.toml");

    let config = match fs::read_to_string(config_file_path).map(|s| toml::from_str(&s)) {
        Ok(Ok(config)) => config,
        _ => Config::default(),
    };
    init_with_config(_td.into(), config)
}

fn init_with_config(type_def: TypeDef, config: Config) {
    let codegen = CodeGen::from(type_def);
    if let Some(_conf) = config.typescript {
        _conf.import_with_extension;
        codegen.typescript();
    }
}
