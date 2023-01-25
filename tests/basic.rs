mod utils;
use databuf::{Decoder, Encoder};
use frpc::procedure;

procedure! {
    user = 1
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
