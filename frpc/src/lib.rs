pub mod fn_once;
pub mod input;
pub mod output;
mod private;
mod state;
// mod rpc;

#[doc(hidden)]
#[cfg(debug_assertions)]
pub mod __private;

pub use frpc_macros::procedure;
pub use frpc_macros::Message;
pub use state::State;

pub const DATABUF_CONFIG: u8 = databuf::config::num::LEB128 | databuf::config::len::BEU30;

#[doc(hidden)]
pub use databuf;

#[doc(hidden)]
pub async fn run<'de, Args, Ret, State>(
    func: impl crate::fn_once::FnOnce<Args, Output = Ret>,
    state: State,
    reader: &mut &'de [u8],
    w: &mut (impl crate::output::AsyncWriter + Unpin + Send),
) -> std::io::Result<()>
where
    Args: crate::input::Input<'de, State>,
    Ret: crate::output::Output,
{
    let args = Args::decode(state, reader).unwrap();
    let output = func.call_once(args);
    Ret::send_output(output, w).await
}

// #[doc(hidden)]
// #[cfg(debug_assertions)]
// pub unsafe fn __codegen(type_def: frpc_message::TypeDef) {
//     use libloading::{Error, Library, Symbol};

//     #[cfg(target_os = "windows")]
//     const CODEGEN_DYLIB: &str = "codegen.dll";
//     #[cfg(target_os = "linux")]
//     const CODEGEN_DYLIB: &str = "codegen.so";
//     #[cfg(target_os = "macos")]
//     const CODEGEN_DYLIB: &str = "codegen.dylib";
//     #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
//     compile_error!("`codegen` is not available on the current operating system.");

//     let path = std::env::var("FRPC_CODEGEN")
//         .unwrap_or_else(|_| format!("{}/lib/{CODEGEN_DYLIB}", env!("CARGO_MANIFEST_DIR")));

//     let mut filename = std::path::PathBuf::from(path);
//     if filename.is_dir() {
//         filename = filename.join(CODEGEN_DYLIB);
//     }
//     let run = || -> Result<_, Error> {
//         let lib = Library::new(&filename)?;
//         let codegen_from: Symbol<unsafe extern "C" fn(*const u8, usize)> = lib.get(b"main\0")?;

//         let bytes = type_def.as_bytes();
//         let len = bytes.len();
//         codegen_from(bytes.as_ptr(), len);
//         Ok(())
//     };
//     if let Err(msg) = run() {
//         eprintln!("[ERROR] {msg}, Path = {filename:?}");
//     }
// }
