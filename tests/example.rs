#![allow(warnings)]
// ---------------------------------------------------------------------

// async fn a(num: u8) -> u8 {
//     123
// }

// rpc! {
//     a = 1
// }

// #[test]
// fn test_name() {
//     // println!("{:#?}", rpc::type_def());
// }

use frpc::{message::Context, Message};

#[derive(Message)]
struct Foo<T = u8>(T);

// #[derive(Message)]
struct C {
    foo: Foo,
}

const _: () = {
    use ::frpc::message as __msg;
    use __msg::_utils::{c, s};
    impl __msg::Message for C {
        fn ty(ctx: &mut __msg::Context) -> __msg::Type {
            let name = ::std::format!("{}::C", ::std::module_path!());
            if let ::std::collections::hash_map::Entry::Vacant(entry) =
                ctx.costom_types.entry(c(&name))
            {
                entry.insert(::std::default::Default::default());
                // let a = __msg::CustomTypeVariant;
                // entry.insert(*a);
            }
            __msg::Type::CustomType(name)
        }
    }
};

// struct Recursive<T> {
//     value: T,
//     next: Box<Recursive<Recursive<T>>>
// }

// #[test]
// fn test_name() {
//     let ctx = Context::default();
//     println!("{:?}", C::ty(&mut ctx));
//     println!("{:?}", ctx);
// }
