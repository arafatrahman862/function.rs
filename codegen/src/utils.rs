use proc_macro::{Ident, TokenTree};
use std::iter::Peekable;

pub fn consume_ident_if(
    input: &mut Peekable<impl Iterator<Item = TokenTree>>,
    text: &str,
) -> Option<Ident> {
    if let Some(TokenTree::Ident(ident)) = input.peek() {
        if ident.to_string() == text {
            unsafe {
                match input.next().unwrap_unchecked() {
                    TokenTree::Ident(i) => return Some(i),
                    _ => core::hint::unreachable_unchecked(),
                }
            }
        }
    }
    None
}
