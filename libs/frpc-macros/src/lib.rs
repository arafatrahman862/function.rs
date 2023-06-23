use proc_macro::TokenStream;
use quote::ToTokens;

mod declare;
mod message;
mod utils;

#[proc_macro_derive(Message)]
pub fn message(input: TokenStream) -> TokenStream {
    message::new(input)
}

#[proc_macro]
pub fn declare(tokens: TokenStream) -> TokenStream {
    syn::parse_macro_input!(tokens as declare::Declare)
        .to_token_stream()
        .into()
}
