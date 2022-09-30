use proc_macro::TokenStream;

#[allow(deprecated)]
use std::hash::SipHasher;
use std::{hash::Hasher, str::FromStr};

#[proc_macro]
pub fn hash_from_ident(input: TokenStream) -> TokenStream {
    #[allow(deprecated)]
    let mut h = SipHasher::new();
    h.write(input.to_string().as_bytes());
    TokenStream::from_str(&h.finish().to_string()).unwrap()
}
