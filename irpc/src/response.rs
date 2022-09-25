use crate::*;
use bin_layout::Encoder;
use std::{
    future::{ready, Ready},
    iter,
    result::Result,
};

pub trait Response {
    type Bytes: AsRef<[u8]>;
    type AsyncResult: Future<Output = Result<Self::Bytes, &'static str>>;
    type Stream: Iterator<Item = Self::AsyncResult>;
    fn into_bytes_stream(&self) -> Self::Stream;
}

impl<T: Encoder> Response for T {
    type Bytes = Vec<u8>;
    type AsyncResult = Ready<Result<Self::Bytes, &'static str>>;
    type Stream = iter::Once<Self::AsyncResult>;

    fn into_bytes_stream(&self) -> Self::Stream {
        iter::once(ready({
            let mut bytes = Vec::new();
            match Encoder::encoder(self, &mut bytes) {
                Err(_) => Err("Parse Error"),
                Ok(_) => Ok(bytes),
            }
        }))
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Display;

    use super::*;
    use bin_layout::Decoder;

    #[tokio::test]
    async fn into_bytes_stream_once() {
        let mut stream = "HelloWorld".into_bytes_stream();
        let mut once = false;
        while let Some(data) = stream.next() {
            if let Ok(bytes) = data.await {
                assert_eq!(String::decode(bytes.as_ref()).unwrap(), "HelloWorld");
            }
            if once {
                unreachable!()
            }
            once = true
        }
    }
}
