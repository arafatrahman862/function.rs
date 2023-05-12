use proc_macro::TokenStream;

#[cfg(not(debug_assertions))]
#[proc_macro_derive(Message)]
pub fn message(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[cfg(debug_assertions)]
use quote::{format_ident, quote, quote_spanned};
#[cfg(debug_assertions)]
use syn::{__private::TokenStream2, spanned::Spanned, *};

#[cfg(debug_assertions)]
#[proc_macro_derive(Message)]
pub fn message(input: TokenStream) -> TokenStream {
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
        return syn::Error::new(
            param.span(),
            "Generic type support isn't complete yet, But it's on our roadmap.",
        )
        .to_compile_error()
        .into();
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let (kind, body) = match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => ("Struct", parse_object(&fields)),
            Fields::Unnamed(fields) => ("Tuple", parse_tuple(&fields)),
            Fields::Unit => panic!("`Message` struct needs at most one field"),
        },
        Data::Enum(mut data) => {
            let is_unit_enum_variant = data
                .variants
                .iter()
                .all(|v| v.discriminant.is_some() || matches!(v.fields, Fields::Unit));

            let variants = data
                .variants
                .iter_mut()
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
                            let body = parse_object(fields);
                            quote! { Struct(::std::vec![#body]) }
                        }
                        Fields::Unnamed(fields) => {
                            let body = parse_tuple(fields);
                            quote! { Tuple(::std::vec![#body]) }
                        }
                        Fields::Unit => quote! { Unit },
                    };
                    quote_spanned! (v.span()=> ___m::EnumField {
                        doc: s(#doc),
                        name: s(#name),
                        kind: ___m::EnumKind::#kind
                    })
                });
                ("Enum", quote! {  #(#recurse),* })
            }
        }
        Data::Union(_) => panic!("`Message` implementation for `union` is not yet stabilized"),
    };

    let kind = format_ident!("{kind}");

    TokenStream::from(quote! {
        const _: () = {
            use ::frpc::__private::frpc_message as ___m;
            use ___m::_utils::{s,c};
            impl #impl_generics ___m::Message for #ident #ty_generics #where_clause {
                fn ty(ctx: &mut ___m::Context) -> ___m::Ty {
                    let name = ::std::format!(#name, ::std::module_path!());
                    if let ::std::collections::btree_map::Entry::Vacant(entry) = ctx.costom_types.entry(c(&name)) {
                        entry.insert(::std::default::Default::default());
                        let costom_type = ___m::CustomType {
                            doc: s(#doc),
                            fields: ::std::vec![#body]
                        };
                        ctx.costom_types.insert(c(&name), ___m::CustomTypeKind::#kind(costom_type));
                    }
                    ___m::Ty::CustomType (name)
                }
            }
        };
    })
}

#[cfg(debug_assertions)]
fn parse_tuple(fields: &FieldsUnnamed) -> TokenStream2 {
    let recurse = fields.unnamed.iter().map(|f| {
        let doc = get_comments_from(&f.attrs);
        let ty = &f.ty;
        quote_spanned! (f.span()=> ___m::TupleField {
            doc: s(#doc),
            ty: <#ty as ___m::Message>::ty(ctx),
        })
    });
    quote! { #(#recurse),* }
}

#[cfg(debug_assertions)]
fn parse_object(fields: &FieldsNamed) -> TokenStream2 {
    let recurse = fields.named.iter().map(|f| {
        let doc = get_comments_from(&f.attrs);
        let name = f.ident.clone().unwrap().to_string();
        let ty = &f.ty;
        quote_spanned! (f.span()=> ___m::StructField {
            doc: s(#doc),
            name: s(#name),
            ty: <#ty as ___m::Message>::ty(ctx)
        })
    });
    quote! { #(#recurse),* }
}

#[cfg(debug_assertions)]
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
#[cfg(debug_assertions)]
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
