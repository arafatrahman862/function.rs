use proc_macro2::{Literal, Punct, Span};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    *,
};

pub enum State {
    #[allow(dead_code)]
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
    _eq_token: Token![=],
    id: Literal,
}

pub struct Service {
    _service_token: Ident,
    ident: Ident,
    _brace_token: token::Brace,
    funcs: Punctuated<Func, Token![,]>,
}

#[derive(Default)]
pub struct Declare {
    pub state: Option<State>,
    pub services: Vec<Service>,
}

macro_rules! err {
    [$span: expr, $msg: expr] => {
        return Err(Error::new($span, $msg))
    };
}

impl Parse for State {
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
                return Err(input.error("todo!"));
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

impl Parse for Service {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let service_token: Ident = input.parse()?;
        if service_token != "service" {
            err!(service_token.span(), "expected `service` keyword");
        }
        Ok(Service {
            _service_token: service_token,
            ident: input.parse()?,
            _brace_token: braced!(content in input),
            funcs: content.parse_terminated(Func::parse, Token![,])?,
        })
    }
}

impl Parse for Func {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Func {
            name: input.parse()?,
            _eq_token: input.parse()?,
            id: input.parse()?,
        })
    }
}

impl Parse for Declare {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut declare = Declare::default();
        while !input.is_empty() {
            if input.peek(Token![type]) {
                let state = input.parse()?;
                if let Some(old_state) = declare.state {
                    let type_value = match old_state {
                        State::Trait { .. } => unreachable!(),
                        State::Type { ty, .. } => ty.into_token_stream().to_string(),
                    };
                    let span = match state {
                        State::Trait { span, .. } | State::Type { span, .. } => span,
                    };
                    err!(
                        span,
                        format!("duplicate definition (State) with value: {}", type_value)
                    );
                }
                declare.state = Some(state);
            } else if input.peek(Ident::peek_any) {
                declare.services.push(input.parse()?);
            } else {
                return Err(input.error("unexpected token"));
            }
        }
        Ok(declare)
    }
}

impl Declare {
    pub fn gen_code(self) -> proc_macro2::TokenStream {
        let state = self
            .state
            .map(|state| match state {
                State::Trait { .. } => unreachable!(),
                State::Type { ty, .. } => quote! { #ty },
            })
            .unwrap_or(quote! { () });

        let services = self.services.iter().map(|Service { ident, funcs, .. }| {
            let use_func = funcs
                .iter()
                .map(|Func { name, .. }| quote_spanned!(name.span()=> use #name ));

            let func_ty = funcs.iter().map(|Func { name, id, .. }| {
                let path = name.to_string();
                quote_spanned!(name.span()=> frpc::__private::fn_ty(&#name, &mut ___c, #id,  #path))
            });

            let func = funcs.iter().map(|Func { name, id, .. }| {
                quote_spanned!(name.span()=>
                    #id => frpc::run(#name, state, &mut reader, w).await
                )
            });        
            quote_spanned!(ident.span()=>
                struct #ident;

                #[cfg(debug_assertions)]
                impl ::std::convert::From<#ident> for ::frpc::__private::frpc_message::TypeDef {
                    fn from(_: #ident) -> Self {
                        #(#use_func;)*
                        let mut ___c = ::frpc::__private::frpc_message::Context::default();
                        let funcs = ::std::vec::Vec::from([#(#func_ty,)*]);
                        Self { ctx: ___c, funcs }
                    }
                }
                
                impl #ident {
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
            )
        });
        quote! { #(#services)* }
    }
}
