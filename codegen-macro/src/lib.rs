use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
// use databuf_derive::Decoder;

// fn a() {
//     databuf_derive::Decoder();
// }

#[proc_macro_derive(Resource)]
pub fn resource(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input as DeriveInput);

    TokenStream::from(quote! {
        impl Resource for #ident {}
    })
}
