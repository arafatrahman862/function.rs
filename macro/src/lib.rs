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
    // let is_generic = generics.type_params().next().is_some();
    let generic_params = generics.type_params().map(|param| param.ident.clone()).collect::<Vec<_>>();
    
    add_trait_bounds(&mut generics, parse_quote!(__msg::Message));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let body = match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => parse_object(&fields),
            Fields::Unnamed(fields) => parse_tuple(&fields),
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
                    quote_spanned! (v.span()=> __msg::UnitField {
                        doc: s(#doc),
                        name: s(#name),
                        value: #value
                    })
                });
                quote! { #(#recurse),* }
            } else {
                let recurse = variants.map(|(doc, name, v)| {
                    let kind = match &mut v.fields {
                        Fields::Named(fields) => {
                            // let _f = fields.named.iter_mut();
                            let body = parse_object(fields);
                            quote! { Struct(::std::vec![#body]) }
                        }
                        Fields::Unnamed(fields) => {
                            // let f =
                            let body = parse_tuple(fields);
                            quote! { Tuple(::std::vec![#body]) }
                        }
                        Fields::Unit => quote! { Unit },
                    };
                    quote_spanned! (v.span()=> __msg::EnumField {
                        doc: s(#doc),
                        name: s(#name),
                        kind: __msg::UnionKind::#kind
                    })
                });
                quote! {  #(#recurse),* }
            }
        }
        Data::Union(_) => panic!("`Message` implementation for `union` is not yet stabilized"),
    };

    let (table, result_expr) = if generic_params.is_empty() {
        (quote!(costom_types), quote!(__msg::Type::CustomType (name)))
    } else {
        let args = generics.type_params().map(|param| {
            let name= &param.ident;
            quote_spanned!(param.span()=> <#name as __msg::Message>::ty(ctx))
        });
        (quote!(generic_costom_types), quote!(__msg::Type::Generic { args: ::std::vec![#(#args),*], name }))
    };

    TokenStream::from(quote! {
        const _: () = {
            use ::frpc::message as __msg;
            use __msg::_utils::{s,c};
            impl #impl_generics __msg::Message for #ident #ty_generics #where_clause {
                fn ty(ctx: &mut __msg::Context) -> __msg::Type {
                    let name = ::std::format!(#name, ::std::module_path!());
                    if let ::std::collections::hash_map::Entry::Vacant(entry) = ctx.#table.entry(c(&name)) {
                        entry.insert(::std::default::Default::default());

                        // let mut obj = __msg::CustomType {
                        //     doc: s(#doc),
                        //     fields: ::std::vec![#body]
                        // };
                        // ctx.def_table.insert(c(&name), obj);
                    }
                    #result_expr
                }
            }
        };
    })
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

fn replace_generic_param(ty: TokenStream2, generic_ty_params: &Vec<Ident>) -> TokenStream2 {
    if !generic_ty_params.is_empty() {
        let tokens = ty.into_iter().map(|tt| match tt {
            quote::__private::TokenTree::Ident(ref ty_param) => generic_ty_params
                .iter()
                .enumerate()
                .find_map(|(idx, name)| {
                    (ty_param == name).then(|| {
                        let name = format_ident!("T{idx}");
                        quote_spanned!(ty_param.span()=> __msg::__generic_param::#name)
                    })
                })
                .unwrap_or(ty_param.to_token_stream()),

            tt => tt.to_token_stream(),
        });
        return quote!(#(#tokens)*);
    }
    ty
}

#[test]
fn test_name() {
    let code = quote!(Box<Node<Vec<T>>>);
    let result = replace_generic_param(code, &vec![format_ident!("U"), format_ident!("T")]);
    println!("{:?}", result.to_string());
}

fn parse_tuple(fields: &FieldsUnnamed) -> TokenStream2 {
    let recurse = fields.unnamed.iter().map(|f| {
        let doc = get_comments_from(&f.attrs);
        let ty = &f.ty;
        quote_spanned! (f.span()=> __msg::TupleStructField {
            doc: s(#doc),
            ty: <#ty as __msg::Message>::ty(def),
        })
    });
    quote! { #(#recurse),* }
}

fn parse_object(fields: &FieldsNamed) -> TokenStream2 {
    let recurse = fields.named.iter().map(|f| {
        let doc = get_comments_from(&f.attrs);
        let name = f.ident.clone().unwrap().to_string();
        let ty = &f.ty;
        quote_spanned! (f.span()=> __msg::StructField {
            doc: s(#doc),
            name: s(#name),
            ty: <#ty as __msg::Message>::ty(def)
        })
    });
    quote! { #(#recurse),* }
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

/// Add a bound `T: Message` to every type parameter of `T`.
fn add_trait_bounds(generics: &mut Generics, bound: TypeParamBound) {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(bound.clone());
        }
    }
}
