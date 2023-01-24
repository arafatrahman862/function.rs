mod utils;
use frpc::{procedure, Decoder, Encoder};

procedure! {
    user = 1
}

pub async fn execute<W>(id: u16, data: Vec<u8>, writer: &mut W) -> ::std::io::Result<()>
where
    W: frpc::tokio::io::AsyncWrite + std::marker::Unpin + Send,
{
    match id {
        1 => {
            let args = frpc::Decoder::decode(&data).unwrap();
            let output = frpc::fn_once::FnOnce::call_once(user, args).await;
            frpc::output::Output::write(&output, writer).await
        }
        _ => {
            return ::std::result::Result::Err(::std::io::Error::new(
                ::std::io::ErrorKind::AddrNotAvailable,
                "Unknown request id",
            ))
        }
    }
}
async fn user(name: String, age: u8) -> String {
    let res = match age {
        ..=18 => "We're excited to have you here!",
        ..=25 => "We're glad you joined us. Hope you find something interesting.",
        _ => "It's great to have you here.",
    };
    format!("Hello {name}! {res}")
}

#[test]
fn test_name() {
    utils::execute_fut(async {
        let mut writer: Vec<u8> = vec![];
        procedure::execute(1, (String::from("Nur"), 22u8).encode(), &mut writer)
            .await
            .unwrap();

        println!("{:?}", String::decode(&writer));
    });
}
