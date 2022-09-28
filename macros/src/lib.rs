use proc_macro::TokenStream;

#[proc_macro]
pub fn rpc(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}