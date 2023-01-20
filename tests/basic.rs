#![allow(warnings)]
use frpc::Message;

// #[derive(Message)]
// struct User {
//     _field1: String,
//     _field2: [u8; 10],
// }

// #[derive(Message)]
// struct User2(u16, u16);

/// wcwec
///wcwec
#[derive(Message)]
enum E {
    D,
    /// adad
    B ,
    C = 4,
}

#[test]
fn test_name() {
    std::file!();   
    std::line!();   
    std::module_path!();   
}