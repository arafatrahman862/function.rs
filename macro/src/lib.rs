use proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::__private::TokenStream2;
use syn::spanned::Spanned;
use syn::*;

#[rustfmt::skip]
#[proc_macro_derive(Message)]
pub fn message(input: TokenStream) -> TokenStream {
    let DeriveInput { attrs, ident, mut generics, data, .. } = parse_macro_input!(input);

    let doc = get_comments_from(&attrs);
    let name = format!("{{}}::{ident}");
    let generic_params = generics.type_params().map(|param| param.ident.clone()).collect::<Vec<_>>();
    
    add_trait_bounds(&mut generics, parse_quote!(___m::Message));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let (kind, body) = match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => ("Struct", parse_object(&fields, &generic_params)),
            Fields::Unnamed(fields) => ("TupleStruct", parse_tuple(&fields, &generic_params)),
            Fields::Unit => panic!("`Message` struct needs at most one field"),
        },
        Data::Enum(mut data) => {
            let is_unit_enum_variant = data.variants.iter()
                .all(|v| v.discriminant.is_some() || matches!(v.fields, Fields::Unit));

            let variants = data.variants.iter_mut()
                .map(|v| (get_comments_from(&v.attrs), v.ident.to_string(), v));

            if is_unit_enum_variant {
                let mut value: isize = -1;
                let recurse = variants.map(|(doc, name, v)| {
                    value = match &v.discriminant {
                        Some((_, expr)) => parse_int(expr),
                        None => value + 1,
                    };
                    quote_spanned! (v.span()=> ___m::UnitField {
                        doc: s(#doc),
                        name: s(#name),
                        value: #value
                    })
                });
                ("Unit", quote! { #(#recurse),* })
            } else {
                let recurse = variants.map(|(doc, name, v)| {
                    let kind = match &mut v.fields {
                        Fields::Named(fields) => {
                            let body = parse_object(fields, &generic_params);
                            quote! { Struct(::std::vec![#body]) }
                        }
                        Fields::Unnamed(fields) => {
                            let body = parse_tuple(fields, &generic_params);
                            quote! { Tuple(::std::vec![#body]) }
                        }
                        Fields::Unit => quote! { Unit },
                    };
                    quote_spanned! (v.span()=> ___m::EnumField {
                        doc: s(#doc),
                        name: s(#name),
                        kind: ___m::UnionKind::#kind
                    })
                });
                ("Enum", quote! {  #(#recurse),* })
            }
        }
        Data::Union(_) => panic!("`Message` implementation for `union` is not yet stabilized"),
    };

    let kind = format_ident!("{kind}");

    let (table, mid, result_expr) = if generic_params.is_empty() {
        (
            quote!(costom_types),
            quote! { ___m::CustomTypeKind::#kind(costom_type) },
            quote!(___m::Type::CustomType (name))
        )
    } else { 
        (
            quote!(generic_costom_types),
            {
                let params = generic_params.iter().map(|param| {
                    let param = param.to_string();
                    quote! { s(#param) }
                });
                quote! {
                    ___m::GenericCustomTypeKind::#kind(___m::Generic { 
                        params: ::std::vec![#(#params),*], 
                        costom_type, 
                    })
                }
            },
            {
                let args = generics.type_params().map(|param| {
                    let name= &param.ident;
                    quote_spanned!(param.span()=> <#name as ___m::Message>::ty(ctx))
                });
                quote!(___m::Type::Generic { args: ::std::vec![#(#args),*], name })
            }
        )
    };

    TokenStream::from(quote! {
        const _: () = {
            use ::frpc::message as ___m;
            use ___m::_utils::{s,c};
            impl #impl_generics ___m::Message for #ident #ty_generics #where_clause {
                fn ty(ctx: &mut ___m::Context) -> ___m::Type {
                    let name = ::std::format!(#name, ::std::module_path!());
                    if let ::std::collections::hash_map::Entry::Vacant(entry) = ctx.#table.entry(c(&name)) {
                        entry.insert(::std::default::Default::default());
                        let costom_type = ___m::CustomType {
                            doc: s(#doc),
                            fields: ::std::vec![#body]
                        };
                        ctx.#table.insert(c(&name), #mid);
                    }
                    #result_expr
                }
            }
        };
    })
}

fn replace_generic_param(ty: TokenStream2, generic_params: &Vec<Ident>) -> TokenStream2 {
    if !generic_params.is_empty() {
        let tokens = ty.into_iter().map(|tt| match tt {
            quote::__private::TokenTree::Ident(ref ty_param) => generic_params
                .iter()
                .enumerate()
                .find_map(|(idx, name)| {
                    (ty_param == name).then(|| {
                        let name = format_ident!("T{idx}");
                        quote_spanned!(ty_param.span()=> ___m::__gp::#name)
                    })
                })
                .unwrap_or_else(|| ty_param.to_token_stream()),

            tt => tt.to_token_stream(),
        });
        return quote!(#(#tokens)*);
    }
    ty
}

fn parse_tuple(fields: &FieldsUnnamed, generic_params: &Vec<Ident>) -> TokenStream2 {
    let recurse = fields.unnamed.iter().map(|f| {
        let doc = get_comments_from(&f.attrs);
        let ty = replace_generic_param(f.ty.to_token_stream(), generic_params);
        quote_spanned! (f.span()=> ___m::TupleStructField {
            doc: s(#doc),
            ty: <#ty as ___m::Message>::ty(ctx),
        })
    });
    quote! { #(#recurse),* }
}

fn parse_object(fields: &FieldsNamed, generic_params: &Vec<Ident>) -> TokenStream2 {
    let recurse = fields.named.iter().map(|f| {
        let doc = get_comments_from(&f.attrs);
        let name = f.ident.clone().unwrap().to_string();
        let ty = replace_generic_param(f.ty.to_token_stream(), generic_params);
        quote_spanned! (f.span()=> ___m::StructField {
            doc: s(#doc),
            name: s(#name),
            ty: <#ty as ___m::Message>::ty(ctx)
        })
    });
    quote! { #(#recurse),* }
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

#[rustfmt::skip]
fn get_comments_from(attrs: &Vec<Attribute>) -> String {
    let mut string = String::new();
    for Attribute { style, path: Path { segments, .. }, tokens, .. } in attrs {
        if let (AttrStyle::Outer, 1, "doc") = (style, segments.len(), segments[0].ident.to_string().as_ref()) {
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

/// Add a bound `T: ___m::Message` to every type parameter of `T`.
fn add_trait_bounds(generics: &mut Generics, bound: TypeParamBound) {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(bound.clone());
        }
    }
}
