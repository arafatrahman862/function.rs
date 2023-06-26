use super::*;

/// It defines the behavior for sending responses over a transport channel.
#[async_trait]
pub trait Transport {
    #[allow(missing_docs)]

    async fn server_stream(
        &mut self,
        mut poll: impl for<'cx, 'c, 'buf> FnMut(&'cx mut Context<'c>, &'buf mut Vec<u8>) -> Poll<bool>
            + Send,
    ) {
        let mut buf = vec![];
        while poll_fn(|cx| poll(cx, &mut buf)).await {
            // ...
        }
        // ...
    }

    async fn unary(
        &mut self,
        mut poll: impl for<'cx, 'c, 'buf> FnMut(&'cx mut Context<'c>, &'buf mut Vec<u8>) -> Poll<()>
            + Send,
    ) {
        let mut buf = vec![];
        poll_fn(|cx| poll(cx, &mut buf)).await;
        todo!()
    }

    async fn unary_sync(
        &mut self,
        cb: impl for<'buf> FnOnce(&'buf mut Vec<u8>) -> databuf::Result<io::Result<()>> + Send,
    ) {
        let mut buf = vec![];
        let _ = cb(&mut buf);
        todo!()
    }
}

// -------------------------------------------------------------------------------

/// It implemented by different types representing various output formats.
#[async_trait]
pub trait Output: crate::output_type::OutputType + Sized {
    /// It produces the output data and sends it over the specified transport.
    async fn produce<TR>(this: databuf::Result<Self>, _: &mut TR)
    where
        TR: Transport + Send;
}

impl<T> Output for Return<T>
where
    T: Send + Encode + TypeId,
{
    fn produce<'tr: 'fut, 'fut, TR>(
        this: databuf::Result<Self>,
        transport: &'tr mut TR,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'fut>>
    where
        TR: 'fut + Transport + Send,
        Self: 'fut,
    {
        transport.unary_sync(|buf| {
            this.map(|data| Encode::encode::<{ crate::DATABUF_CONFIG }>(&data.0, buf))
        })
    }
}

impl<Fut> Output for Fut
where
    Fut: Future + Send,
    Fut::Output: Encode + TypeId,
{
    fn produce<'tr: 'fut, 'fut, TR>(
        this: databuf::Result<Self>,
        transport: &'tr mut TR,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'fut>>
    where
        TR: 'fut + Transport + Send,
        Self: 'fut,
    {
        transport.unary(|cx, buf| {
            // println!("{:?}", buf);
            // let data = this?.await;
            // Ok(Encode::encode::<{ crate::DATABUF_CONFIG }>(&data, buf))
            todo!()
        })
    }
}

// ------------------------------------------------------

#[async_trait]
impl<G> Output for SSE<G>
where
    G: AsyncGenerator + Send,
{
    async fn produce<TR>(this: databuf::Result<Self>, transport: &mut TR)
    where
        TR: Transport + Send,
    {
        let mut gen = pin!(this.unwrap().0);
        transport
            .server_stream(|cx, buf| {
                gen.as_mut().poll_resume(cx).map(|s| match s {
                    GeneratorState::Yielded(val) => {
                        Encode::encode::<{ crate::DATABUF_CONFIG }>(&val, buf).unwrap();
                        false
                    }
                    GeneratorState::Complete(val) => {
                        Encode::encode::<{ crate::DATABUF_CONFIG }>(&val, buf).unwrap();
                        true
                    }
                })
            })
            .await;
    }
}
