#![allow(warnings)]
use frpc::{
    message::{Context, Message},
    Message,
};

#[derive(Message)]
struct Foo<K, V = u8>(K, V);

// #[derive(Message)]
// struct C {
//     foo: Foo,
// }

// #[derive(Message)]
struct Recursive<T> {
    value: T,
    next: Box<Recursive<Recursive<T>>>,
}

const _: () = {
    use ::frpc::message as ___m;
    use ___m::_utils::{c, s};
    impl<T: ___m::Message> ___m::Message for Recursive<T> {
        fn ty(ctx: &mut ___m::Context) -> ___m::Type {
            let name = ::std::format!("{}::Recursive", ::std::module_path!());
            
            if let ::std::collections::hash_map::Entry::Vacant(entry) =
                ctx.generic_costom_types.entry(c(&name))
            {
                entry.insert(::std::default::Default::default());

                let costom_type = ___m::CustomType {
                    doc: s(""),
                    fields: ::std::vec![
                        ___m::StructField {
                            doc: s(""),
                            name: s("value"),
                            ty: <___m::__gp::T0 as ___m::Message>::ty(ctx)
                        },
                        ___m::StructField {
                            doc: s(""),
                            name: s("next"),
                            // ty: <Box<Recursive<Recursive<___m::__gp::T0>>> as ___m::Message>::ty(
                            //     ctx
                            // )
                            ty: <Box<Recursive<u8>> as ___m::Message>::ty(
                                ctx
                            )
                        }
                    ],
                };
                ctx.generic_costom_types.insert(
                    c(&name),
                    ___m::GenericCustomTypeKind::Struct(___m::Generic {
                        params: ::std::vec![s("T")],
                        costom_type,
                    }),
                );
            }
            ___m::Type::Generic {
                args: ::std::vec![<T as ___m::Message>::ty(ctx)],
                name,
            }
        }
    }
};

#[test]
fn test_name() {
    let mut ctx = Context::default();
    println!("{:?}", <Recursive::<u8> as Message>::ty(&mut ctx));
    println!("{:#?}", ctx);
}
