use crate::utils::ToToken;
use proc_macro::TokenStream;
use quote::{quote, quote_each_token};
use syn::{spanned::Spanned, *};

pub fn new(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs,
        ident,
        generics,
        data,
        ..
    } = parse_macro_input!(input);

    let doc = get_comments_from(&attrs);
    let name = format!("{{}}::{ident}");

    if let Some(param) = generics.type_params().next() {
        return Error::new(
            param.span(),
            "Support for generic type isn't complete yet, But it's on our roadmap.",
        )
        .to_compile_error()
        .into();
    }

    let kind = match &data {
        Data::Struct(data) => match data.fields {
            Fields::Named(_) => "Struct",
            Fields::Unnamed(_) => "Tuple",
            Fields::Unit => panic!("`Message` struct needs at most one field"),
        },
        Data::Enum(data) => {
            if data
                .variants
                .iter()
                .all(|v| v.discriminant.is_some() || matches!(v.fields, Fields::Unit))
            {
                "Unit"
            } else {
                "Enum"
            }
        }
        Data::Union(_) => panic!("`Message` implementation for `union` is not yet stabilized"),
    };

    let body = ToToken(|mut tokens| match &data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => to_object(fields, tokens),
            Fields::Unnamed(fields) => to_tuple(fields, tokens),
            Fields::Unit => unreachable!(),
        },
        Data::Enum(data) => {
            let variants = data
                .variants
                .iter()
                .map(|v| (get_comments_from(&v.attrs), v.ident.to_string(), v));

            match kind {
                "Unit" => {
                    let mut value: isize = -1;
                    for (doc, name, v) in variants {
                        value = match &v.discriminant {
                            Some((_, expr)) => parse_int(expr),
                            None => value + 1,
                        };
                        quote_each_token! {tokens
                            ___m::UnitField {
                                doc: s(#doc),
                                name: s(#name),
                                value: #value
                            },
                        }
                    }
                }
                "Enum" => {
                    for (doc, name, v) in variants {
                        let kind = ToToken(|mut tokens| match &v.fields {
                            Fields::Named(fields) => to_object(fields, tokens),
                            Fields::Unnamed(fields) => to_tuple(fields, tokens),
                            Fields::Unit => quote::quote_token!(Unit tokens),
                        });
                        quote_each_token! {tokens
                            ___m::EnumField {
                                doc: s(#doc),
                                name: s(#name),
                                kind: ___m::EnumKind::#kind
                            },
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        Data::Union(_) => unreachable!(),
    });

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    TokenStream::from(quote! {
        const _: () = {
            use ::frpc::__private::frpc_message as ___m;
            use ___m::_utils::{ s, c };
            impl #impl_generics ___m::Message for #ident #ty_generics #where_clause {
                fn ty(__costom_types: &mut ___m::CostomTypes) -> ___m::Ty {
                    let name = ::std::format!(#name, ::std::module_path!());
                    if let ::std::collections::btree_map::Entry::Vacant(entry) = __costom_types.entry(c(&name)) {
                        entry.insert(::std::default::Default::default());
                        let costom_type = ___m::CustomType {
                            doc: s(#doc),
                            fields: ::std::vec![#body]
                        };
                        __costom_types.insert(c(&name), ___m::CustomTypeKind::#kind(costom_type));
                    }
                    ___m::Ty::CustomType (name)
                }
            }
        };
    })
}

fn to_tuple(fields: &FieldsUnnamed, mut tokens: &mut proc_macro2::TokenStream) {
    for field in &fields.unnamed {
        let doc = get_comments_from(&field.attrs);
        let ty = &field.ty;
        quote_each_token! {tokens
            ___m::TupleField {
                doc: s(#doc),
                ty: <#ty as ___m::Message>::ty(__costom_types)
            },
        }
    }
}

fn to_object(fields: &FieldsNamed, mut tokens: &mut proc_macro2::TokenStream) {
    for field in &fields.named {
        let doc = get_comments_from(&field.attrs);
        let name = match &field.ident {
            Some(ident) => ident.to_string(),
            None => String::new(),
        };
        let ty = &field.ty;
        quote_each_token! {tokens
            ___m::StructField {
                doc: s(#doc),
                name: s(#name),
                ty: <#ty as ___m::Message>::ty(__costom_types)
            },
        }
    }
}

fn parse_int(expr: &Expr) -> isize {
    match expr {
        Expr::Lit(expr_lit) => match &expr_lit.lit {
            Lit::Int(int) => int.base10_parse().unwrap(),
            _ => panic!("Expect integer"),
        },
        _ => panic!("Not a number"),
    }
}

fn get_comments_from(attrs: &Vec<Attribute>) -> String {
    let mut string = String::new();
    for attr in attrs {
        let segments = &attr.path().segments;
        if let (AttrStyle::Outer, 1, "doc") = (
            attr.style,
            segments.len(),
            segments[0].ident.to_string().as_ref(),
        ) {
            // string += tokens.to_string().trim_start_matches('=').trim_start().trim_matches('"');
            string += "\n";
        }
    }
    string
}
