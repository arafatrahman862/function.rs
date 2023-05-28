mod code_writer;
pub mod config;

use config::Config;
use frpc_codegen::CodeGen;
use frpc_message::TypeDef;

use std::{fmt::Write, fs};

type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

pub fn init(service: impl Into<TypeDef>) {
    let type_def = service.into();
    let config_file_path = config::manifest_dir().join("codegen.toml");

    let config = match fs::read_to_string(config_file_path).map(|s| toml::from_str(&s)) {
        Ok(Ok(config)) => config,
        _ => Config::default(),
    };
    if let Some(err) = init_with_config(type_def, config).err() {
        eprintln!("{err:#?}");
        std::process::exit(1);
    }
}

fn init_with_config(type_def: TypeDef, config: Config) -> Result {
    let writer = code_writer::CodeWriter::from(type_def);
    if let Some(config) = config.typescript {
        writer.generate_typescript_binding(config)?;
    }
    Ok(())
}
