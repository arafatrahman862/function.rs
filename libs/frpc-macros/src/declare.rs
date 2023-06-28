use proc_macro2::{Literal, Punct, Span};
use quote::{quote_each_token, ToTokens, TokenStreamExt};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    *,
};

use crate::utils::ToToken;

pub enum State {
    // Trait {
    //     span: Span,
    //     params: Punctuated<TypeParamBound, Token![+]>,
    // },
    Type { span: Span, ty: Type },
}

pub struct Func {
    name: Ident,
    _eq_token: Token![=],
    id: Literal,
}

pub struct Service {
    attrs: Vec<Attribute>,
    vis: Visibility,
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

// ------------------------------------------------------------------------------

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
            // ':' => {
            //     todo!()
            // }
            '=' => Self::Type {
                span,
                ty: input.parse()?,
            },
            _ => err!(punct.span(), "expected `=`"),
        };
        input.parse::<Token![;]>()?;
        Ok(state)
    }
}

impl Parse for Service {
    fn parse(input: ParseStream) -> Result<Self> {
        let err_msg = "expected `service` keyword";
        let content;
        Ok(Service {
            attrs: input.call(Attribute::parse_outer)?,
            vis: input.parse()?,
            _service_token: match input.parse::<Ident>() {
                Err(e) => err!(e.span(), err_msg),
                Ok(e) if e != "service" => err!(e.span(), err_msg),
                Ok(token) => token,
            },
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
                        State::Type { ty, .. } => ty.into_token_stream().to_string(),
                    };
                    let span = match state {
                        State::Type { span, .. } => span,
                    };
                    err!(
                        span,
                        format!("duplicate definition (State) with value: {}", type_value)
                    );
                }
                declare.state = Some(state);
            } else if input.peek(Ident::peek_any) || input.peek(Token![#]) {
                declare.services.push(input.parse()?);
            } else {
                return Err(input.error("unexpected token"));
            }
        }
        Ok(declare)
    }
}

impl ToTokens for Declare {
    fn to_tokens(&self, mut tokens: &mut proc_macro2::TokenStream) {
        let Self { state, services } = self;

        let state = ToToken(|mut tokens| match state {
            Some(State::Type { ty, .. }) => ty.to_tokens(tokens),
            None => quote::quote_token!(() tokens),
        });

        for Service {
            attrs,
            vis,
            ident,
            funcs,
            ..
        } in services
        {
            let import_funcs = ToToken(|mut tokens| {
                for Func { name, .. } in funcs {
                    quote_each_token!(tokens
                        use #name;
                    );
                }
            });
            let func_types = ToToken(|mut tokens| {
                for Func { name, id, .. } in funcs {
                    let path = name.to_string();
                    quote_each_token!(tokens
                        ::frpc::__private::fn_sig(&#name, &mut __costom_types, #id,  #path),
                    );
                }
            });
            let funcs = ToToken(|mut tokens| {
                for Func { name, id, .. } in funcs {
                    quote_each_token!(tokens
                        #id => Some(Output::produce(#name, state, cursor, transport)),
                    );
                }
            });

            let name = ident.to_string();
            tokens.append_all(attrs);

            quote_each_token!(tokens
                #vis struct #ident;

                #[cfg(debug_assertions)]
                impl ::std::convert::From<#ident> for ::frpc::__private::frpc_message::TypeDef {
                    fn from(_: #ident) -> Self {
                        #import_funcs
                        let mut __costom_types = ::frpc::__private::frpc_message::CostomTypes::default();
                        let funcs = ::std::vec::Vec::from([#func_types]);
                        Self::new(#name, __costom_types, funcs)
                    }
                }

                impl #ident {
                    pub fn execute<'fut, TR: ::frpc::Transport + ::std::marker::Send>(
                        state: #state,
                        id: u16,
                        cursor: &'fut mut &[u8],
                        transport: &'fut mut TR,
                    ) -> ::std::option::Option<::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = ()> + ::std::marker::Send + 'fut>>>
                    {
                        use ::std::option::Option::{Some, None};
                        use ::frpc::Output;
                        match id {
                            #funcs
                            _ => None
                        }
                    }
                }
            );
        }
    }
}
