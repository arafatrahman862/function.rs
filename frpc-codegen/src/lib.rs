use std::io::{Seek, SeekFrom, Write};
use std::{env, fs::File, io::Read};

use frpc_message::TypeDef;

pub mod javascript;

pub mod fmt;
pub mod utils;

#[macro_export]
macro_rules! fmt {
    (box $c: expr) => { crate::fmt::Fmt::<Box<dyn Fn(&mut std::fmt::Formatter) -> std::fmt::Result>>::new(Box::new($c)) };
    ($c: expr) => { crate::fmt::Fmt::new($c) };

    (type $lt: lifetime) => { crate::fmt::Fmt<impl Fn(&mut std::fmt::Formatter) -> std::fmt::Result + $lt> };
    (type) => { crate::fmt::Fmt<impl Fn(&mut std::fmt::Formatter) -> std::fmt::Result> };
}

// --------------------------------------------------------------
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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
            let _ = file
                .seek(SeekFrom::Start(0))
                .and_then(|_| file.write_all(&hash.to_le_bytes()));
        }
        Err(err) => eprintln!("{err}"),
    }
    let _ = TypeDef::try_from(bytes).map(codegen);
}

fn prev_hash() -> Result<(File, u64), Box<dyn std::error::Error>> {
    let var_key = "CARGO_PKG_NAME";
    let name = env::var(var_key).map_err(|err| format!("[ERROR] {var_key}: {err}"))?;
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

pub fn codegen(_type_def: TypeDef) {
    println!("{:?}", "sfssad");
    println!("{:?}", _type_def.funcs.len());
}

/*
 // create_dir_all(path).unwrap();
        // let mut log = File::options()
        //     .create(true)
        //     .append(true)
        //     .open(path.join("codegen.log"))
        //     .unwrap();

        // let _ = writeln!(log, "[ERROR] {msg}, Path = {}", filename.display());
*/
