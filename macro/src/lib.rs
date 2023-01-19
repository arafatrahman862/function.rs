use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, *};

#[proc_macro_derive(Message)]
pub fn message(input: TokenStream) -> TokenStream {
    let DeriveInput {
        data,
        ident,
        mut generics,
        ..
    } = parse_macro_input!(input);

    add_trait_bounds(&mut generics, parse_quote!(Message));

    let (_, ty_generics, where_clause) = generics.split_for_impl();
    let (struct_type, body) = match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let ty = &f.ty;
                    quote_spanned! {f.span()=> (
                        String::from(stringify!(#name)),
                        <#ty as Message>::ty()
                    )}
                });
                (quote!(Struct), quote! { #(#recurse),* })
            }
            Fields::Unnamed(fields) => {
                let recurse = fields.unnamed.iter().map(|f| {
                    let ty = &f.ty;
                    quote_spanned! {f.span()=> <#ty as Message>::ty() }
                });
                (quote!(TupleStruct), quote! {#(#recurse),*})
            }
            Fields::Unit => panic!("`Message` struct needs at most one field"),
        },
        _ => panic!("Default `Encoder` implementation for `enum` not yet stabilized"),
    };

    TokenStream::from(quote! {
        const _: () = {
            use ::std::prelude::v1::*;
            use ::frpc::{Message, message::Type};
            impl #generics Message for #ident #ty_generics #where_clause {
                fn ty() -> Type {
                    Type::#struct_type {
                        name: String::from(stringify!(#ident)),
                        fields: vec![#body],
                    }
                }
            }
        };
    })
}

/// Add a bound `T: Message` to every type parameter of `T`.
fn add_trait_bounds(generics: &mut Generics, bound: TypeParamBound) {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(bound.clone());
        }
    }
}
