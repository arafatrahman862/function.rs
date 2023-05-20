#![allow(warnings)]
mod http;

pub mod transport {
    use std::future::Future;

    pub trait Unary {
        type Future<'a>: Future<Output = std::io::Result<()>> + 'a
        where
            Self: 'a;

        fn send_unary_response(&mut self, _: Box<[u8]>) -> Self::Future<'_>;
    }

    pub trait Transport: Unary {}

    impl Unary for Vec<u8> {
        type Future<'a> = std::future::Ready<std::io::Result<()>>
        where
            Self: 'a;

        fn send_unary_response(&mut self, _: Box<[u8]>) -> Self::Future<'_> {
            std::future::ready(Ok(()))
        }
    }
}
