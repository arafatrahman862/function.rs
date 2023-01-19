#![allow(warnings)]
use frpc::Message;

#[derive(Message)]
struct User {
    _field1: String,
    _field2: [u8; 10],
}

#[derive(Message)]
struct User2(u16, u16);

enum E {
    B,
    C,
    D,
}
