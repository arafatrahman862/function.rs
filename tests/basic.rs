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

#[derive(Message, Decoder, Encoder)]
enum Foo {
    Quz { x: u8 },
    Bar(u8, Bez),
    Many((Vec<Foo>, Vec<Foo>)),
}

#[derive(Message, Decoder, Encoder)]
struct Bez(u8, u16);

#[derive(Message, Decoder, Encoder)]
struct User {
    name: String,
    age: u8,
    car: Car,
    foo: Foo,
}

async fn get_user(user: (u8, User)) -> (u8, User) {
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
    let code = frpc_codegen::javascript::code::generate(&typedef);
    let code = format!("{}", code);
    println!("{code}");
    // std::fs::write("play.ts", format!("{}", code));

    // println!("{typedef:#?}");
    // utils::execute_fut(async {
    //     let mut writer: Vec<u8> = vec![];
    //     procedure::execute(1, (String::from("Nur"), 22u8).encode(), &mut writer)
    //         .await
    //         .unwrap();
    //     println!("{:?}", String::decode(&writer));
    // });
}
