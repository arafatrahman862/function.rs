pub struct ToToken<F: Fn(&mut proc_macro2::TokenStream)>(pub F);

impl<F> quote::ToTokens for ToToken<F>
where
    F: Fn(&mut proc_macro2::TokenStream),
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0(tokens);
    }
}
