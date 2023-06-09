mod code_writer;
pub mod config;

use config::Config;
use frpc_codegen::CodeGen;
use frpc_message::TypeDef;

use std::{fmt::Write, fs};

type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init(service: impl Into<TypeDef>) {
    let config = CONFIG.get_or_init(|| {
        let config_path = config::manifest_dir().join("codegen.toml");
        match fs::read_to_string(config_path).map(|s| toml::from_str(&s)) {
            Ok(Ok(config)) => config,
            _ => Config::default(),
        }
    });
    if let Some(err) = init_with_config(service.into(), config).err() {
        eprintln!("{err:#?}");
        std::process::exit(1);
    }
}

fn init_with_config(type_def: TypeDef, config: &Config) -> Result {
    let writer = code_writer::CodeWriter::from(type_def);
    if let Some(config) = &config.typescript {
        writer.generate_typescript_binding(config)?;
    }
    Ok(())
}
