#![allow(warnings)]
use proc_macro::*;
use virtue::prelude::*;

#[proc_macro_attribute]
pub fn handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut out = item.to_string();
    let ref mut input = item.into_iter();

    // Ignore everything until `fn` keyword
    let _ = input.skip_while(|tt| tt.to_string() != "fn").next();
    let mut name = input.next().unwrap().to_string();

    match input.next().unwrap() {
        TokenTree::Group(group) => {}
        tt => panic!("Function can't have generics"),
    }

    // ----------------------------------------------------
    // let pos = out.find("(").unwrap();
    // out.insert_str(pos + 1, "req:Request,");
    // let ts: TokenStream = out.parse().unwrap();
    // ----------------------------------------------------

    name.insert_str(0, "pub struct");
    name.push_str("{}");
    // gen_handler_struct(name);

    "".parse().unwrap()
}

fn gen_handler_struct(name: String) -> TokenStream {
    format!("
        struct {name} {}
        impl {name} {{
            fn call(req: Request) -> impl Responder {{
                
            }}
        }}
    ").parse().unwrap()
}

#[proc_macro]
pub fn ctx(item: TokenStream) -> TokenStream {
    // log(item);
    "".parse().unwrap()
}

fn log<T: std::fmt::Debug + std::fmt::Display>(item: T) {
    std::fs::write("diff", item.to_string()).unwrap();
    std::fs::write("diff.tt", format!("{:#?}", item)).unwrap();
}


