use std::path::Path;

use super::*;

pub struct CodeWriter {
    codegen: CodeGen,
}

impl From<TypeDef> for CodeWriter {
    fn from(td: TypeDef) -> Self {
        Self {
            codegen: CodeGen::from(td),
        }
    }
}

impl CodeWriter {
    pub fn generate_typescript_binding(&self, config: &config::typescript::Config) -> Result {
        fs::create_dir_all(&config.out_dir)?;

        let prelude_path = config.out_dir.join("databuf.lib.ts");
        if !prelude_path.exists() {
            let typescript_modules = Path::new(std::file!())
                .parent()
                .unwrap()
                .join("../client/typescript/");

            fs::copy(typescript_modules.join("databuf.ts"), prelude_path)?;
            fs::copy(
                typescript_modules.join("http.transport.ts"),
                config.out_dir.join("http.transport.ts"),
            )?;
        }
        let ext = match config.preserve_import_extension {
            true => ".ts",
            false => "",
        };
        let mut code = format!("import* as use from './databuf.lib{ext}'\n");
        write!(code, "{}", self.codegen.typescript())?;
        let filename = format!("{}.ts", self.codegen.type_def.name);
        Ok(fs::write(config.out_dir.join(filename), code)?)
    }
}
