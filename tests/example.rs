#![allow(warnings)]
use frpc::{
    message::{Context, Message},
    Message,
};

// #[derive(Message)]
// struct Foo<K, V = u8>(K, V);

// // #[derive(Message)]
// // struct C {
// //     foo: Foo,
// // }

// // #[derive(Message)]
// struct Recursive<T> {
//     value: T,
//     next: Box<Recursive<Recursive<T>>>,
// }

// #[test]
// fn test_name() {
//     let mut ctx = Context::default();
//     println!("{:?}", <Recursive::<u8> as Message>::ty(&mut ctx));
//     println!("{:#?}", ctx);
// }
