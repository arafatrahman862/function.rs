// #![allow(warnings)]
// mod utils;
// use databuf::{Decode, Encode};
// use frpc::{procedure, Message};

// #[derive(Message, Decode, Encode)]
// enum Car {
//     Foo,
//     Bar,
// }

// #[derive(Message, Decode, Encode)]
// enum Foo {
//     Quz { x: u8 },
//     Bar(u8, Bez),
//     Many((Vec<Foo>, Vec<Foo>)),
// }

// #[derive(Message, Decode, Encode)]
// struct Bez(u8, u16);

// #[derive(Message, Decode, Encode)]
// struct User {
//     name: String,
//     age: u8,
//     car: Car,
//     foo: Foo,
// }

// // ---------------------------------------------------------

// procedure! {
//     type State: S;
//     rpc A {
//         get_user = 2,
//         demo = 3,
//         user = 6,
//     }
// }
// async fn demo(s: ()) {}

// async fn get_user(user: (u8, User)) -> (u8, User) {
//     user
// }

// /// Hello World
// async fn user(name: String, age: Unum) -> String {
//     let res = match age {
//         ..=18 => "We're excited to have you here!",
//         ..=25 => "We're glad you joined us. Hope you find something interesting.",
//         _ => "It's great to have you here.",
//     }; // 335 | 457
//     format!("Hello {name}! {res}")
// }

// type Unum = u16;

// // #[test]
// // fn test_name() {
// //     build_codegen();
// //     // procedure::codegen();
// //     std::thread::sleep(std::time::Duration::from_secs(5))
// // }
// // fn build_codegen() {
// //     let mut f = std::process::Command::new("cargo")
// //         .args(["build", "--lib", "--package", "codegen", "--release"])
// //         .spawn()
// //         .unwrap();
// //     f.wait();
// // }
