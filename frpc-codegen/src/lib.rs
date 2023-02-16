pub mod fmt;
mod interface_path;
pub mod javascript;
pub mod utils;

use frpc_message::TypeDef;
use interface_path::InterfacePath;
use std::{
    collections::hash_map::DefaultHasher,
    env,
    fs::File,
    hash::{Hash, Hasher},
    io::{Read, Seek, SeekFrom, Write},
};

fn prev_hash() -> Result<(File, u64), Box<dyn std::error::Error>> {
    let key = "CARGO_PKG_NAME";
    let name = env::var(key).map_err(|msg| format!("[ERROR] {key}: {msg}"))?;
    let path = env::temp_dir().join(format!("frpc_codegen_{name}.hex"));

    let mut file = File::options()
        .create(true)
        .read(true)
        .write(true)
        .open(path)?;

    let mut buf = [0; 8];
    file.read(&mut buf)?;
    Ok((file, u64::from_le_bytes(buf)))
}

#[no_mangle]
pub unsafe extern "C" fn codegen_from(raw_bytes: *const u8, len: usize) {
    let bytes = std::slice::from_raw_parts(raw_bytes, len);
    match prev_hash() {
        Ok((mut file, prev_hash)) => {
            let hash = {
                let mut hasher = DefaultHasher::new();
                bytes.hash(&mut hasher);
                hasher.finish()
            };
            if hash == prev_hash {
                // println!("[SKIP]");
                return;
            }
            if let Err(msg) = file
                .seek(SeekFrom::Start(0))
                .and_then(|_| file.write_all(&hash.to_le_bytes()))
            {
                eprintln!("[ERROR] {msg}");
            }
        }
        Err(err) => eprintln!("{err}"),
    }
    let _ = TypeDef::try_from(bytes).map(codegen);
}

pub struct Provider<'a> {
    type_def: &'a TypeDef,
    input_paths: Vec<&'a String>,
    output_paths: Vec<&'a String>,
}

pub fn codegen(type_def: TypeDef) {
    let mut input = InterfacePath::new(&type_def.ctx);
    let mut output = InterfacePath::new(&type_def.ctx);

    input.add_tys(type_def.funcs.iter().flat_map(|func| func.args.iter()));
    output.add_tys(type_def.funcs.iter().map(|func| &func.retn));

    let provider = Provider {
        input_paths: input.paths,
        output_paths: output.paths,
        type_def: &type_def,
    };

    javascript::generate(&provider);
}