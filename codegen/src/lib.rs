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
    path::PathBuf,
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
    file.read_exact(&mut buf)?;
    Ok((file, u64::from_le_bytes(buf)))
}

/// # Safety
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
                .rewind()
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

    if let Err(err) = javascript_codegen(&provider) {
        eprintln!("[ERROR] {err:?}")
    }
}

const JS_PRELUDE: &[u8] = include_bytes!("../../lib/typescript/databuf.ts");

fn manifest_dir() -> PathBuf {
    env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .or_else(|_| env::current_dir())
        .unwrap_or("./".into())
}

fn javascript_codegen(provider: &Provider) -> std::io::Result<()> {
    let path = manifest_dir().join("/target/frpc/");
    std::fs::create_dir_all(&path)?;

    let file_path = path.join(env::var("CARGO_PKG_NAME").unwrap_or("frpc.stub".into()) + ".ts");
    let mut file = File::options().create(true).write(true).open(&file_path)?;

    // if prelude_len > metadata.len() {
    //     println!("ReRun...");
    //     file.write_all(JS_PRELUDE)?;
    // }

    let prelude_len = JS_PRELUDE.len() as u64;
    let offset = file.seek(SeekFrom::Start(prelude_len))?;

    file.write_all(javascript::generate(provider).to_string().as_bytes())?;
    file.set_len(offset + prelude_len)
}


#[test]
fn test_name() {
    
}