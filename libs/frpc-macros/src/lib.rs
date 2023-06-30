use proc_macro::TokenStream;
use quote::{quote, ToTokens};

mod declare;
use syn::{parse_macro_input, DeriveInput};
use type_id_derive_impl::utils;

#[proc_macro_derive(Message)]
pub fn message(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let mut output = proc_macro2::TokenStream::new();
    type_id_derive_impl::expend(
        quote! { ::frpc::__private::frpc_message::type_id },
        &input,
        &mut output,
    );
    TokenStream::from(output)
}

#[proc_macro]
pub fn declare(tokens: TokenStream) -> TokenStream {
    syn::parse_macro_input!(tokens as declare::Declare)
        .to_token_stream()
        .into()
}
