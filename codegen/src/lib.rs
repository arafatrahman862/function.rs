mod conf;
pub mod fmt;
mod interface_path;
pub mod javascript;
mod logger;
pub mod utils;

use frpc_message::TypeDef;
pub use interface_path::InterfacePath;
// use log::{error, info};
// use logger::CodegenLogger;
// use std::{
//     collections::hash_map::DefaultHasher,
//     env,
//     fs::{self, File},
//     hash::{Hash, Hasher},
//     io::{Read, Result, Seek, SeekFrom, Write},
// };

// fn prev_hash() -> Result<(File, u64)> {
//     let path = env::temp_dir().join(conf::pkg_name() + "_frpc_codegen.hex");
//     let mut file = File::options()
//         .create(true)
//         .read(true)
//         .write(true)
//         .open(path)?;

//     let mut buf = [0; 8];
//     let _ = file.read(&mut buf)?;
//     Ok((file, u64::from_le_bytes(buf)))
// }

// /// # Safety
// #[no_mangle]
// pub unsafe extern "C" fn main(raw_bytes: *const u8, len: usize) {
//     if !log::log_enabled!(log::Level::Info) {
//         if let Ok(logger) = CodegenLogger::new(conf::manifest_dir().join("codegen.log")) {
//             log::set_max_level(log::LevelFilter::Info);
//             log::set_boxed_logger(Box::new(logger)).unwrap();
//         }
//     }

//     let bytes = std::slice::from_raw_parts(raw_bytes, len);
//     match prev_hash() {
//         Ok((mut file, prev_hash)) => {
//             let hash = {
//                 let mut hasher = DefaultHasher::new();
//                 bytes.hash(&mut hasher);
//                 hasher.finish()
//             };
//             if hash == prev_hash {
//                 return log::trace!("[SKIP] {hash}");
//             }
//             if let Err(msg) = file
//                 .rewind()
//                 .and_then(|_| file.write_all(&hash.to_le_bytes()))
//             {
//                 error!("{msg}");
//             }
//         }
//         Err(err) => error!("{err}"),
//     }
//     if let Err(msg) = TypeDef::try_from(bytes).map(codegen) {
//         error!("Unable to parse `TypeDef`, {msg}");
//     }
// }

pub struct CodeGen<'a> {
    type_def: &'a TypeDef,
    input_paths: Vec<&'a String>,
    output_paths: Vec<&'a String>,
}

impl<'a> CodeGen<'a> {
    pub fn new(type_def: &'a TypeDef) -> Self {
        let mut input = InterfacePath::new(&type_def.ctx);
        let mut output = InterfacePath::new(&type_def.ctx);

        input.add_tys(type_def.funcs.iter().flat_map(|func| func.args.iter()));
        output.add_tys(type_def.funcs.iter().map(|func| &func.retn));

        Self {
            type_def,
            input_paths: input.paths,
            output_paths: output.paths,
        }
    }
}

// fn javascript_codegen(provider: &Provider) -> Result<()> {
//     const JS_PRELUDE: &[u8] = include_bytes!("../../lib/typescript/databuf.ts");

//     let dir = conf::manifest_dir().join("target").join("frpc");
//     fs::create_dir_all(&dir)?;

//     let path = dir.join(conf::pkg_name() + ".ts");
//     let code = javascript::generate(provider).to_string();

//     let write_full = || {
//         info!("[JavaScript] Update: {path:?}");
//         let bytes = [JS_PRELUDE, code.as_bytes()].concat();
//         let mut file = File::options().create(true).write(true).open(&path)?;
//         file.write_all(&bytes)?;
//         file.set_len(bytes.len() as u64)
//     };

//     if !path.is_file() {
//         write_full()?;
//         let mut perm = path.metadata()?.permissions();
//         perm.set_readonly(true);
//         return fs::set_permissions(path, perm);
//     }

//     // -----------------------------------------------------------

//     let mut perm = path.metadata()?.permissions();
//     if perm.readonly() {
//         perm.set_readonly(false);
//         fs::set_permissions(&path, perm.clone())?;

//         let prelude_len = JS_PRELUDE.len() as u64;
//         let mut file = File::options().write(true).open(&path)?;

//         let offset = file.seek(SeekFrom::Start(prelude_len))?;
//         file.write_all(code.as_bytes())?;
//         file.set_len(offset + prelude_len)?;
//     } else {
//         write_full()?;
//     }
//     perm.set_readonly(true);
//     fs::set_permissions(path, perm)
// }
