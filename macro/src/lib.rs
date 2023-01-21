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

    add_trait_bounds(&mut generics, parse_quote!(__msg::Message));

    let (_, ty_generics, where_clause) = generics.split_for_impl();
    let (variant, variant_field, body) = match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => ("Struct", "structs", parse_object(&fields)),
            Fields::Unnamed(fields) => ("TupleStruct", "tuple_structs", parse_tuple(&fields)),
            Fields::Unit => panic!("`Message` struct needs at most one field"),
        },
        Data::Enum(data) => {
            let is_enum = data
                .variants
                .iter()
                .all(|v| v.discriminant.is_some() || matches!(v.fields, Fields::Unit));

            let variants = data
                .variants
                .iter()
                .map(|v| (get_comments_from(&v.attrs), v.ident.to_string(), v));

            if is_enum {
                let mut value: isize = -1;
                let recurse = variants.map(|(doc, name, v)| {
                    value = match &v.discriminant {
                        Some((_, expr)) => match expr {
                            Expr::Lit(expr_lit) => match &expr_lit.lit {
                                Lit::Int(int) => int.base10_parse().unwrap(),
                                _ => panic!("Expect integer"),
                            },
                            _ => panic!("Not a number"),
                        },
                        None => value + 1,
                    };
                    quote_spanned! (v.span()=> __msg::EnumField {
                        doc: s(#doc),
                        name: s(#name),
                        value: #value
                    })
                });
                ("Enum", "enums", quote! { #(#recurse),* })
            } else {
                let recurse = variants.map(|(doc, name, v)| {
                    let kind = match &v.fields {
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
                    quote_spanned! (v.span()=> __msg::UnionField {
                        doc: s(#doc),
                        name: s(#name),
                        kind: __msg::UnionKind::#kind
                    })
                });
                ("Union", "unions", quote! {  #(#recurse),* })
            }
        }
        Data::Union(_) => panic!("`Message` implementation for `union` is not yet stabilized"),
    };
    let doc = get_comments_from(&attrs);
    let name = format!("{{}}::{ident}");
    let variant = format_ident!("{variant}");
    let variant_field = format_ident!("{variant_field}");
    TokenStream::from(quote! {
        const _: () = {
            use ::frpc::message as __msg;
            use __msg::_utils::{s,c};
            impl #generics __msg::Message for #ident #ty_generics #where_clause {
                fn ty(def: &mut __msg::Definition) -> __msg::Type {
                    let name = ::std::format!(#name, ::std::module_path!());
                    if let ::std::collections::hash_map::Entry::Vacant(entry) = def.structs.entry(c(&name)) {
                        entry.insert(__msg::CostomType::new());
                        let mut obj = __msg::CostomType {
                            doc: s(#doc),
                            fields: ::std::vec![#body]
                        };
                        def.#variant_field.insert(c(&name), obj);
                    }
                    __msg::Type::#variant (name)
                }
            }
        };
    })
}

fn parse_tuple(fields: &FieldsUnnamed) -> __private::TokenStream2 {
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

fn parse_object(fields: &FieldsNamed) -> __private::TokenStream2 {
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

fn get_comments_from(attrs: &Vec<Attribute>) -> String {
    let mut string = String::new();
    for Attribute {
        style,
        path: Path { segments, .. },
        tokens,
        ..
    } in attrs
    {
        if let (AttrStyle::Outer, 1, "doc") = (
            style,
            segments.len(),
            segments[0].ident.to_string().as_ref(),
        ) {
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
