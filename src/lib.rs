pub mod fn_once;
pub mod output;

#[doc(hidden)]
#[cfg(debug_assertions)]
pub mod util;

#[doc(hidden)]
#[cfg(debug_assertions)]
pub use frpc_message;

pub use frpc_macros::Message;

pub const DATABUF_CONF: u8 = databuf::config::num::LEB128 | databuf::config::len::LEU29;

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
        #[cfg(debug_assertions)]
        pub fn codegen() {
            ::std::thread::spawn(move || unsafe { $crate::__codegen(self::type_def()) });
        }

        #[allow(dead_code)]
        #[cfg(not(debug_assertions))]
        pub fn codegen() {}

        pub async fn execute<W>(id: u16, data: Vec<u8>, writer: &mut W) -> ::std::io::Result<()>
        where
            W: ::tokio::io::AsyncWrite + ::std::marker::Unpin + ::std::marker::Send,
        {
            match id {
                $($id => {
                    let args = ::databuf::Decode::from_bytes::<{$crate::DATABUF_CONF}>(&data).unwrap();
                    let output = $crate::fn_once::FnOnce::call_once($func, args).await;
                    $crate::output::Output::write(&output, writer).await
                }),*
                _ => {
                    return ::std::result::Result::Err(::std::io::Error::new(
                        ::std::io::ErrorKind::AddrNotAvailable,
                        "Unknown ID",
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

    let filename = std::env::var("FRPC_CODEGEN")
        .unwrap_or_else(|_| format!("{}/target/frpc/{CODEGEN_DYLIB}", env!("CARGO_MANIFEST_DIR")));

    let run = || -> Result<_, Error> {
        let lib = Library::new(&filename)?;
        let codegen_from: Symbol<unsafe extern "C" fn(*const u8, usize)> =
            lib.get(b"codegen_from\0")?;

        let bytes = type_def.as_bytes();
        let len = bytes.len();
        codegen_from(bytes.as_ptr(), len);
        Ok(())
    };
    if let Err(msg) = run() {
        eprintln!("[ERROR] {msg}, Path = {filename}");
    }
}

#[test]
fn build_codegen() {
    let time = std::time::Instant::now();
    println!("Building...");
    let _ = std::process::Command::new("cargo")
        .args(["build", "--lib", "--package", "codegen"])
        .output()
        .expect("failed to execute process");

    println!("Done: {:?} sec\n", time.elapsed().as_secs());
    std::env::set_var("FRPC_CODEGEN", "target/debug/codegen.dll");
}

#[test]
fn test_name() {
    build_codegen();
    unsafe {
        __codegen(frpc_message::TypeDef {
            ctx: Default::default(),
            funcs: Default::default(),
        });
    }
}