#![allow(warnings)]
use frpc::__private::frpc_message::TypeDef;
use frpc::declare;

async fn foo() {}
async fn bar() {}

// declare! {
//     service Foo {
//         foo = 15,
//         bar = 14,
//     }
//     service Bar {
//         bar = 14,
//     }
// }

struct Foo;

#[cfg(debug_assertions)]
impl ::std::convert::From<Foo> for ::frpc::__private::frpc_message::TypeDef {
    fn from(_: Foo) -> Self {
        use bar;
        use foo;
        let mut ___c = ::frpc::__private::frpc_message::CostomTypes::default();
        let funcs = ::std::vec::Vec::from([
            ::frpc::__private::fn_ty(&foo, &mut ___c, 15, "foo"),
            ::frpc::__private::fn_ty(&bar, &mut ___c, 14, "bar"),
        ]);
        Self {
            costom_types: ___c,
            funcs,
        }
    }
}
fn main() {
    // let type_def = Bar.into();
    // let codegen = CodeGen::new(&type_def);
    // std::fs::write("./out.ts", format!("{:?}", codegen.javascript()));
}
