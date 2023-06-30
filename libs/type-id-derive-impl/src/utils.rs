use syn::__private::TokenStream2 as TokenStream;

pub struct ToToken<F: Fn(&mut TokenStream)>(pub F);

impl<F> quote::ToTokens for ToToken<F>
where
    F: Fn(&mut TokenStream),
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0(tokens);
    }
}
