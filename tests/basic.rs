#![allow(warnings)]
mod utils;
use databuf::{Decoder, Encoder};
use frpc::{procedure, Message};
use frpc_codegen::code_formatter;

procedure! {
    user = 1
    get_user = 2
}
#[derive(Message, Decoder)]
enum Car {
    Foo,
    Bar
}

#[derive(Message, Decoder)]
struct User {
    name: String,
    age: u8,
    car: Car
}

async fn get_user() -> User {
    User {
        name: "alex".into(),
        age: 20,
        car: Car::Bar
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
    let typedef = procedure::type_def();
    let mut c = code_formatter::CodeFormatter::new();
    frpc_codegen::javascript::code::generate(&mut c, &typedef).unwrap();
    println!("{}", c.buf);
    
    // println!("{typedef:#?}");

    // utils::execute_fut(async {
    //     let mut writer: Vec<u8> = vec![];
    //     procedure::execute(1, (String::from("Nur"), 22u8).encode(), &mut writer)
    //         .await
    //         .unwrap();

    //     println!("{:?}", String::decode(&writer));
    // });
}
