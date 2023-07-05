use proc_macro2::{Literal, Punct, TokenStream};
use quote2::{
    proc_macro2::{self, Span},
    quote, Quote,
};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    *,
};

pub enum State {
    // Trait { params: Punctuated<TypeParamBound, Token![+]> },
    Type { ty: Type },
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
        let punct: Punct = input.parse()?;
        let state = match punct.as_char() {
            // ':' => {}
            '=' => Self::Type { ty: input.parse()? },
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
                if let Some(_) = declare.state {
                    err!(Span::call_site(), format!("duplicate definition (State)"));
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

pub fn expend(output: &mut TokenStream, Declare { state, services }: Declare) {
    let state = quote(|o| match &state {
        Some(State::Type { ty }) => o.add_tokens(ty),
        None => o.add_group('(', |_| {}),
    });

    for Service {
        attrs,
        vis,
        ref ident,
        ref funcs,
        ..
    } in services
    {
        let import_funcs = quote(|o| {
            for Func { name, .. } in funcs {
                quote!(o, {
                    use #name;
                });
            }
        });
        let func_types = quote(|o| {
            for Func { name, id, .. } in funcs {
                let path = name.to_string();
                quote!(o, {
                    ::frpc::__private::fn_sig(&#name, &mut __costom_types, #id,  #path),
                });
            }
        });
        let funcs = quote(|o| {
            for Func { name, id, .. } in funcs {
                quote!(o, {
                    #id => Some(Output::produce(#name, state, cursor, transport)),
                });
            }
        });

        let name = ident.to_string();
        attrs.into_iter().for_each(|attr| output.add_tokens(attr));
        quote!(output, {
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
        });
    }
}
