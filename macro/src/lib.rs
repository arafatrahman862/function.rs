use proc_macro::TokenStream;
use quote::quote;
use syn::*;


#[proc_macro_derive(Message)]
pub fn message(input: TokenStream) -> TokenStream {
    let DeriveInput {
        data,
        ident,
        mut generics,
        ..
    } = parse_macro_input!(input);

    trait_bounds(&mut generics, parse_quote!(databuf::Encoder));

    TokenStream::from(quote! {
        impl Resource for #ident {}
    })
}

/// Add a bound `T: Message` to every type parameter of `T`.
fn trait_bounds(generics: &mut Generics, bound: TypeParamBound) {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(bound.clone());
        }
    }
}