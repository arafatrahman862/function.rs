use std::future::Future;

pub trait Sealed {} // Users in other crates cannot name this trait.

impl<Fut, T: databuf::Encode> Sealed for Fut where Fut: Future<Output = T> {}
