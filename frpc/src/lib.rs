mod ctx;
pub mod fn_once;

#[doc(hidden)]
#[cfg(debug_assertions)]
pub mod util;

#[doc(hidden)]
#[cfg(debug_assertions)]
pub use frpc_message;

pub use ctx::Ctx;
pub use frpc_macros::Message;

pub const DATABUF_CONFIG: u8 = databuf::config::num::LEB128 | databuf::config::len::BEU30;

#[doc(hidden)]
pub use databuf;

#[macro_export]
macro_rules! procedure {
    [$($func:path = $id:literal)*] => (mod procedure {
        use super::*;

        #[allow(dead_code)]
        #[cfg(debug_assertions)]
        pub fn type_def() -> $crate::frpc_message::TypeDef {
            let mut ctx = $crate::frpc_message::Context::default();
            let funcs = vec![
                $({
                    let (args, retn) = $crate::util::async_fn_ty(&$func, &mut ctx);
                    $crate::frpc_message::Func { index: $id, path: stringify!($func).into(), args, retn }
                }),*
            ];
            $crate::frpc_message::TypeDef {
                ctx,
                funcs,
            }
        }

        #[allow(dead_code)]
        #[cfg(not(debug_assertions))]
        pub fn codegen() {}

        #[allow(dead_code)]
        #[cfg(debug_assertions)]
        pub fn codegen() {
            ::std::thread::spawn(move || unsafe { $crate::__codegen(self::type_def()) });
        }

        pub async fn execute<T>(ctx: $crate::Ctx<T>, id: u16, data: Vec<u8>) -> ::std::io::Result<()> {
            match id {
                $($id => {
                    let args = $crate::databuf::Decode::from_bytes::<{$crate::DATABUF_CONFIG}>(&data).unwrap();
                    let output = $crate::fn_once::FnOnce::call_once($func, args).await;
                    todo!()
                }),*
                _ => {
                    return ::std::result::Result::Err(::std::io::Error::new(
                        ::std::io::ErrorKind::AddrNotAvailable,
                        "unknown id",
                    ))
                }
            }
        }
    });
}

#[doc(hidden)]
#[cfg(debug_assertions)]
pub unsafe fn __codegen(type_def: frpc_message::TypeDef) {
    use libloading::{Error, Library, Symbol};

    #[cfg(target_os = "windows")]
    const CODEGEN_DYLIB: &str = "codegen.dll";
    #[cfg(target_os = "linux")]
    const CODEGEN_DYLIB: &str = "codegen.so";
    #[cfg(target_os = "macos")]
    const CODEGEN_DYLIB: &str = "codegen.dylib";
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    compile_error!("`codegen` is not available on the current operating system.");

    let path = std::env::var("FRPC_CODEGEN")
        .unwrap_or_else(|_| format!("{}/lib/{CODEGEN_DYLIB}", env!("CARGO_MANIFEST_DIR")));

    let mut filename = std::path::PathBuf::from(path);
    if filename.is_dir() {
        filename = filename.join(CODEGEN_DYLIB);
    }
    let run = || -> Result<_, Error> {
        let lib = Library::new(&filename)?;
        let codegen_from: Symbol<unsafe extern "C" fn(*const u8, usize)> = lib.get(b"main\0")?;

        let bytes = type_def.as_bytes();
        let len = bytes.len();
        codegen_from(bytes.as_ptr(), len);
        Ok(())
    };
    if let Err(msg) = run() {
        eprintln!("[ERROR] {msg}, Path = {filename:?}");
    }
}
