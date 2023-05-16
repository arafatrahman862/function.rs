mod declare;
#[cfg(debug_assertions)]
mod message;

use proc_macro::TokenStream;

#[cfg(debug_assertions)]
#[proc_macro_derive(Message)]
pub fn message(input: TokenStream) -> TokenStream {
    message::new(input)
}

#[proc_macro_derive(Noop)]
pub fn noop(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro]
pub fn declare(tokens: TokenStream) -> TokenStream {
    syn::parse_macro_input!(tokens as declare::Declare)
        .gen_code()
        .into()
}
