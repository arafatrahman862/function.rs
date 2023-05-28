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
    pub fn generate_typescript_binding(&self, config: config::typescript::Config) -> Result {
        fs::create_dir_all(&config.out_dir)?;

        let prelude = include_bytes!("../client/typescript/databuf.ts");
        let prelude_path = config.out_dir.join("databuf.lib.ts");
        if !prelude_path.exists() {
            fs::write(prelude_path, prelude)?;
        }

        let ext = match config.import_with_extension {
            true => ".ts",
            false => "",
        };
        let mut code = format!("import {{ use }} from './databuf.lib{ext}'\n");
        write!(code, "{}", self.codegen.typescript())?;
        let filename = format!("{}.ts", self.codegen.type_def.name);
        Ok(fs::write(config.out_dir.join(filename), code)?)
    }
}
