use proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{spanned::Spanned, *};

#[proc_macro_derive(Message)]
pub fn message(input: TokenStream) -> TokenStream {
    let DeriveInput {
        data,
        ident,
        mut generics,
        attrs,
        ..
    } = parse_macro_input!(input);

    add_trait_bounds(&mut generics, parse_quote!(Message));

    let (_, ty_generics, where_clause) = generics.split_for_impl();
    let (variant, body) = match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = f.ident.clone().unwrap().to_string();
                    let ty = &f.ty;
                    quote_spanned! (f.span()=> StructField {
                        name: String::from(#name),
                        ty: <#ty as Message>::ty()
                    })
                });
                ("Struct", quote! { #(#recurse),* })
            }
            Fields::Unnamed(fields) => {
                let recurse = fields.unnamed.iter().map(|f| {
                    let ty = &f.ty;
                    quote_spanned! (f.span()=> TupleStructField {
                        ty: <#ty as Message>::ty(),
                    })
                });
                ("TupleStruct", quote! {#(#recurse),*})
            }
            Fields::Unit => panic!("`Message` struct needs at most one field"),
        },
        Data::Enum(enum_data) => {
            let is_enum = enum_data
                .variants
                .iter()
                .all(|v| v.discriminant.is_some() || matches!(v.fields, Fields::Unit));

            if is_enum {
                let mut num: isize = -1;
                let recurse = enum_data.variants.iter().map(|v| {
                    let name = v.ident.to_string();
                    num = match &v.discriminant {
                        Some((_, expr)) => match expr {
                            Expr::Lit(expr_lit) => match &expr_lit.lit {
                                Lit::Int(int) => int.base10_parse().unwrap(),
                                _ => panic!("Expect integer"),
                            },
                            _ => panic!("Not a number"),
                        },
                        None => num + 1,
                    };
                    let doc = get_comments_from(&v.attrs);
                    quote_spanned! (v.span()=> EnumField {
                        doc: String::from(#doc),
                        name: String::from(#name),
                        value: #num
                    })
                });
                ("Enum", quote! { #(#recurse),* })
            } else {
                ("Union", quote! {})
            }
        }
        Data::Union(_) => panic!("`Message` implementation for `union` is not yet stabilized"),
    };
    let doc = get_comments_from(&attrs);
    let name = ident.to_string();
    let variant = format_ident!("{variant}");
    TokenStream::from(quote! {
        const _: () = {
            use ::std::string::String;
            use ::frpc::message::*;
            impl #generics Message for #ident #ty_generics #where_clause {
                fn ty() -> Type {
                    Type::#variant {
                        doc: String::from(#doc),
                        name: String::from(#name),
                        fields: vec![#body],
                    }
                }
            }
        };
    })
}

fn get_comments_from(attrs: &Vec<Attribute>) -> String {
    let mut string = String::new();
    for Attribute { style, path: Path { segments: s, .. }, tokens, .. } in attrs {
        if let (AttrStyle::Outer, 1, "doc") = (style, s.len(), s[0].ident.to_string().as_ref()) {
            string += tokens
                .to_string()
                .trim_start_matches('=')
                .trim_start()
                .trim_matches('"');

            string += "\n";
        }
    }
    string
}

/// Add a bound `T: Message` to every type parameter of `T`.
fn add_trait_bounds(generics: &mut Generics, bound: TypeParamBound) {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(bound.clone());
        }
    }
}
