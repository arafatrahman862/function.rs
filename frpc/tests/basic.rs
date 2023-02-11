#![allow(warnings)]
mod utils;
use databuf::{Decoder, Encoder};
use frpc::{procedure, Message};

procedure! {
    user = 6
    demo = 3
    get_user = 2
}

async fn demo(s: ()) {}

#[derive(Message, Decoder, Encoder)]
enum Car {
    Foo,
    Bar,
}

#[derive(Message, Decoder)]
enum Foo {
    Quz { x: u8 },
    Bar(u8, u16, Bez),
}

#[derive(Message, Decoder)]
struct Bez(u8, u16);

#[derive(Message, Decoder)]
struct User {
    name: String,
    age: u8,
    car: Car,
    foo: Foo,
}

async fn get_user(user: User) -> User {
    user
}

/// Hello World
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
    // let typedef = procedure::;
    let typedef = procedure::type_def();
    println!("{}", frpc_codegen::javascript::code::generate(&typedef));

    // println!("{typedef:#?}");
    // utils::execute_fut(async {
    //     let mut writer: Vec<u8> = vec![];
    //     procedure::execute(1, (String::from("Nur"), 22u8).encode(), &mut writer)
    //         .await
    //         .unwrap();
    //     println!("{:?}", String::decode(&writer));
    // });
}
