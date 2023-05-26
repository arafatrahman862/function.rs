#![allow(warnings)]
use frpc::__private::frpc_message::TypeDef;
use frpc::declare;

async fn foo() {}
async fn bar() {}

declare! {
    type State = u8;

    service Foo {
        foo = 15,
        bar = 14,
    }
    service Bar {
        bar = 14,
    }
}

fn main() {
    // let type_def = Bar.into();
    // let codegen = CodeGen::new(&type_def);
    // std::fs::write("./out.ts", format!("{:?}", codegen.javascript()));
}
