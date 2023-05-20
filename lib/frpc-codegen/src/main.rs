// use std::{
//     env,
//     io::{Read, Result},
//     net::TcpListener,
// };

// use frpc_message::TypeDef;

// const ADDR: &str = "127.0.0.1:4323";
// const HELP: &str = "
// HELP:
//     codegen
// ";

// fn main() {
//     if let None = cli() {
//         print!("{HELP}");
//     }
// }

// fn cli() -> Option<()> {
//     let mut args = env::args();
//     match args.nth(1).as_deref()? {
//         "run" | "-r" => eprintln!("{:#?}", run().err().unwrap()),
//         _ => return None,
//     }
//     Some(())
// }

// fn run() -> Result<()> {
//     let listener = TcpListener::bind(ADDR)?;
//     loop {
//         let (mut stream, _addr) = listener.accept()?;

//         let mut buf = Vec::new();
//         stream.read_to_end(&mut buf)?;
//         let Ok(type_def) = TypeDef::try_from(&buf) else { continue };
//         let _s = codegen::CodeGen::new(&type_def);
//     }
// }

// // impl<'a> Provider<'a> {
// //     // const JS_PRELUDE: &[u8] = include_bytes!("../../lib/typescript/databuf.ts");

// //     fn javascript_codegen(&self) -> Result<()> {
// //         Ok(())
// //     }
// // }

// // fn javascript_codegen(provider: &Provider) -> Result<()> {
// //     const JS_PRELUDE: &[u8] = include_bytes!("../../lib/typescript/databuf.ts");

// //     let dir = conf::manifest_dir().join("target").join("frpc");
// //     fs::create_dir_all(&dir)?;

// //     let path = dir.join(conf::pkg_name() + ".ts");
// //     let code = javascript::generate(provider).to_string();

// //     let write_full = || {
// //         info!("[JavaScript] Update: {path:?}");
// //         let bytes = [JS_PRELUDE, code.as_bytes()].concat();
// //         let mut file = File::options().create(true).write(true).open(&path)?;
// //         file.write_all(&bytes)?;
// //         file.set_len(bytes.len() as u64)
// //     };

// //     if !path.is_file() {
// //         write_full()?;
// //         let mut perm = path.metadata()?.permissions();
// //         perm.set_readonly(true);
// //         return fs::set_permissions(path, perm);
// //     }

// //     // -----------------------------------------------------------

// //     let mut perm = path.metadata()?.permissions();
// //     if perm.readonly() {
// //         perm.set_readonly(false);
// //         fs::set_permissions(&path, perm.clone())?;

// //         let prelude_len = JS_PRELUDE.len() as u64;
// //         let mut file = File::options().write(true).open(&path)?;

// //         let offset = file.seek(SeekFrom::Start(prelude_len))?;
// //         file.write_all(code.as_bytes())?;
// //         file.set_len(offset + prelude_len)?;
// //     } else {
// //         write_full()?;
// //     }
// //     perm.set_readonly(true);
// //     fs::set_permissions(path, perm)
// // }

fn main() {}
