pub mod utils;
use quote::{quote_each_token, ToTokens};
use syn::{
    __private::{Span, TokenStream2 as TokenStream},
    spanned::Spanned,
    *,
};
use utils::ToToken;

pub fn expend(crate_path: impl ToTokens, input: &DeriveInput, mut output: &mut TokenStream) {
    let DeriveInput {
        attrs,
        ident,
        generics,
        data,
        ..
    } = input;

    let doc = get_comments_from(attrs);
    let name = format!("{{}}::{ident}");
    // compile_error!("awd");

    if let Some(param) = generics.type_params().next() {
        return output.extend(
            Error::new(
                param.span(),
                "Support for generic type isn't complete yet, But it's on our roadmap.",
            )
            .to_compile_error(),
        );
    }

    let kind = match data {
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

    let body = ToToken(|mut tokens| match data {
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
                            Some((_, expr)) => parse_int(&expr),
                            None => value + 1,
                        };
                        quote_each_token! {tokens
                            __crate::UnitField::new(#doc, #name, #value),
                        }
                    }
                }
                "Enum" => {
                    for (doc, name, v) in variants {
                        let kind = ToToken(|mut tokens| match &v.fields {
                            Fields::Named(fields) => {
                                let body = ToToken(|tokens| to_object(fields, tokens));
                                quote_each_token! {tokens
                                    Struct(::std::vec![#body])
                                }
                            }
                            Fields::Unnamed(fields) => {
                                let body = ToToken(|tokens| to_tuple(fields, tokens));
                                quote_each_token! {tokens
                                    Tuple(::std::vec![#body])
                                }
                            }
                            Fields::Unit => quote::quote_token!(Unit tokens),
                        });
                        quote_each_token! {tokens
                            __crate::EnumField::new(#doc, #name, __crate::EnumKind::#kind),
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        Data::Union(_) => unreachable!(),
    });
    let kind = Ident::new(kind, Span::call_site());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote_each_token! {output
        const _: () = {
            use #crate_path as __crate;
            impl #impl_generics __crate::TypeId for #ident #ty_generics #where_clause {
                fn ty(__c: &mut __crate::CostomTypes) -> __crate::Ty {
                    __c.register(
                        ::std::format!(#name, ::std::module_path!()),
                        |__c| __crate::CustomTypeKind::#kind(__crate::CustomType::new(#doc, ::std::vec![#body]))
                    )
                }
            }
        };
    }
}

fn to_tuple(fields: &FieldsUnnamed, mut tokens: &mut TokenStream) {
    for field in &fields.unnamed {
        let doc = get_comments_from(&field.attrs);
        let ty = &field.ty;
        quote_each_token! {tokens
            __crate::TupleField::new(#doc, <#ty as __crate::TypeId>::ty(__c)),
        }
    }
}

fn to_object(fields: &FieldsNamed, mut tokens: &mut TokenStream) {
    for field in &fields.named {
        let doc = get_comments_from(&field.attrs);
        let name = match &field.ident {
            Some(ident) => ident.to_string(),
            None => String::new(),
        };
        let ty = &field.ty;
        quote_each_token! {tokens
            __crate::StructField::new(#doc, #name, <#ty as __crate::TypeId>::ty(__c)),
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
        if let Meta::NameValue(MetaNameValue { path, value, .. }) = &attr.meta {
            if path.is_ident("doc") {
                if let Expr::Lit(expr) = value {
                    if let Lit::Str(data) = &expr.lit {
                        string += &data.value();
                        string += "\n"
                    }
                }
            }
        }
    }
    string
}
