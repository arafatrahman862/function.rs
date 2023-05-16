use proc_macro2::{Literal, Punct, Span, TokenTree};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    *,
};

pub enum RpcState {
    Trait {
        span: Span,
        params: Punctuated<TypeParamBound, Token![+]>,
    },
    Type {
        span: Span,
        ty: Type,
    },
}

pub struct Func {
    name: Ident,
    eq_token: Token![=],
    id: Literal,
}

pub struct Export {
    rpc_token: Ident,
    ident: Ident,
    brace_token: token::Brace,
    fields: Punctuated<Func, Token![,]>,
}

#[derive(Default)]
pub struct Rpc {
    pub state: Option<RpcState>,
    pub export: Option<Export>,
}

macro_rules! err {
    [$span: expr, $msg: expr] => {
        return Err(Error::new($span, $msg))
    };
}

impl Parse for RpcState {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![type]>()?;
        let name: Ident = input.parse()?;
        if name != "State" {
            err!(
                name.span(),
                format!("unknown `{name}` keyword, did you mean `State` ?")
            );
        }
        let span = name.span();
        let punct: Punct = input.parse()?;
        let state = match punct.as_char() {
            ':' => {
                return Err(input.error("todo"));
                // Self::Trait {
                //     span,
                //     params: input.parse_terminated(TypeParamBound::parse, Token![+])?,
                // }
            }
            '=' => Self::Type {
                span,
                ty: input.parse()?,
            },
            _ => err!(punct.span(), "expected `:` or `=`"),
        };
        input.parse::<Token![;]>()?;
        Ok(state)
    }
}

impl Parse for Export {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let rpc_token: Ident = input.parse()?;
        if rpc_token != "rpc" {
            err!(rpc_token.span(), "expected `rpc`");
        }
        Ok(Export {
            rpc_token,
            ident: input.parse()?,
            brace_token: braced!(content in input),
            fields: content.parse_terminated(Func::parse, Token![,])?,
        })
    }
}

impl Parse for Func {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Func {
            name: input.parse()?,
            eq_token: input.parse()?,
            id: input.parse()?,
        })
    }
}

impl Parse for Rpc {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut rpc = Rpc::default();
        while !input.is_empty() {
            if input.peek(Token![type]) {
                let rpc_state = input.parse()?;
                if let Some(state) = rpc.state {
                    let type_value = match state {
                        RpcState::Trait { params: name, .. } => "".to_string(),
                        RpcState::Type { ty, .. } => ty.into_token_stream().to_string(),
                    };
                    let span = match rpc_state {
                        RpcState::Trait { span, .. } | RpcState::Type { span, .. } => span,
                    };
                    err!(
                        span,
                        format!("duplicate definition (State) with value: {}", type_value)
                    );
                }
                rpc.state = Some(rpc_state);
            } else if input.peek(Ident::peek_any) {
                let funcs: Export = input.parse()?;
                if let Some(export) = rpc.export {
                    err!(
                        export.ident.span(),
                        format!("duplicate definition (rpc)`, `{}` present`", funcs.ident)
                    );
                }
                rpc.export = Some(funcs)
            } else {
                return Err(input.error("unexpected token"));
            }
        }
        Ok(rpc)
    }
}

impl Rpc {
    pub fn gen_code(self) -> proc_macro2::TokenStream {
        let Some(export) = self.export else { return proc_macro2::TokenStream::new() };
        let rpc_name = export.ident;

        let func = export.fields.iter().map(|Func { name, id, .. }| {
            quote_spanned!(name.span()=>
                #id => frpc::run(super::#name, state, &mut reader, w).await
            )
        });

        let use_func = export
            .fields
            .iter()
            .map(|Func { name, .. }| quote_spanned!(name.span()=> use super::#name ));

        let func_ty = export.fields.iter().map(|Func { name, id, .. }| {
            let path = name.to_string();
            quote_spanned!(name.span()=> frpc::__private::fn_ty(&#name, &mut ___c, #id,  #path))
        });

        let state = self
            .state
            .map(|state| match state {
                RpcState::Trait { .. } => quote! { () },
                RpcState::Type { span, ty } => quote! { super::#ty },
            })
            .unwrap_or(quote! { () });

        quote! {
            #[allow(non_snake_case)]
            mod #rpc_name {
                #[allow(dead_code)]
                pub fn type_def() -> impl ::std::any::Any {
                    #[cfg(debug_assertions)]
                    {
                        let mut ___c = frpc::__private::frpc_message::Context::default();
                        #(#use_func;)*
                        let funcs = ::std::boxed::Box::from([#(#func_ty,)*]);
                        frpc::__private::frpc_message::TypeDef {
                            ctx: ___c,
                            funcs,
                        }
                    }
                }

                pub async fn execute<W>(state: #state, id: u16, data: Box<[u8]>, w: &mut W) -> ::std::io::Result<()>
                where
                    W: ::frpc::output::AsyncWriter + Unpin + Send,
                {
                    let mut reader = &*data;
                    match id {
                        #(#func,)*
                        _ => {
                            return ::std::result::Result::Err(::std::io::Error::new(
                                ::std::io::ErrorKind::AddrNotAvailable,
                                "unknown id",
                            ))
                        }
                    }
                }
            }
        }
    }
}
