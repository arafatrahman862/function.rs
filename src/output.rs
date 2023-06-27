use super::*;

/// It defines the behavior for sending responses over a transport channel.
#[async_trait]
pub trait Transport {
    async fn server_stream(
        &mut self,
        mut poll: impl for<'cx, 'c, 'buf> FnMut(
                &'cx mut Context<'c>,
                &'buf mut Vec<u8>,
            ) -> Poll<io::Result<bool>>
            + Send,
    ) {
        let mut buf = vec![];
        while let Ok(_) = poll_fn(|cx| poll(cx, &mut buf)).await {
            // ...
        }
        // ...
    }

    async fn unary(
        &mut self,
        mut poll: impl for<'cx, 'c, 'buf> FnMut(
                &'cx mut Context<'c>,
                &'buf mut Vec<u8>,
            ) -> Poll<io::Result<()>>
            + Send,
    ) {
        let mut buf = vec![];
        let _ = poll_fn(|cx| poll(cx, &mut buf)).await;
    }

    async fn unary_sync(
        &mut self,
        cb: impl for<'buf> FnOnce(&'buf mut Vec<u8>) -> io::Result<()> + Send,
    ) {
        let mut buf = vec![];
        let _ = cb(&mut buf);
        todo!()
    }
}

// -------------------------------------------------------------------------------

/// It implemented by different types representing various output formats.
#[async_trait]
pub trait Output: crate::output_type::OutputType {
    /// It produces the output data and sends it over the specified transport.
    async fn produce<'de, State, Args>(
        func: impl std_lib::FnOnce<Args, Output = Self> + Send,
        state: State,
        reader: &mut &'de [u8],
        transport: &mut (impl Transport + Send),
    ) where
        State: Send,
        Args: input::Input<'de, State> + Send;
}

impl<T> Output for Return<T>
where
    T: Send + Encode + TypeId,
{
    fn produce<'de, 'reader, 'transport, 'fut, State, Args>(
        func: impl 'fut + std_lib::FnOnce<Args, Output = Self> + Send,
        state: State,
        reader: &'reader mut &'de [u8],
        transport: &'transport mut (impl 'fut + Transport + Send),
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'fut>>
    where
        'de: 'fut,
        'reader: 'fut,
        'transport: 'fut,
        Self: 'fut,
        State: 'fut + Send,
        Args: 'fut + input::Input<'de, State>,
    {
        transport.unary_sync(|buf| match Args::decode(state, reader) {
            Ok(args) => {
                let this = func.call_once(args);
                Encode::encode::<{ crate::DATABUF_CONFIG }>(&this.0, buf)
            }
            Err(err) => Err(io::Error::new(io::ErrorKind::InvalidInput, err)),
        })
    }
}

/// # What is wrong with this code ???
///
/// ```rust
/// let mut state = match Args::decode(state, reader) {
///     Ok(args) => Ok(func.call_once(args)),
///     Err(error) => Err(Some(io::Error::new(io::ErrorKind::InvalidInput, error))),
/// };
/// transport.unary(move |cx, buf| match state {
///     Ok(ref mut fut) => unsafe { Pin::new_unchecked(fut) }
///         .poll(cx)
///         .map(|data| Encode::encode::<{ crate::DATABUF_CONFIG }>(&data, buf)),
///     Err(ref mut err) => {
///         Poll::Ready(Err(err.take().expect("unary()` polled after completion")))
///     }
/// })
/// ```
///
/// I am blindly assuming is that, Beacouse `Args::decode(...)` is outside,
/// The future is created outside on the stack, And then move into `transport.unary(|..| {})` closure.
/// So if Future is large then `Output::produce()` function also become large.
///
/// If `Output::produce()` is large then, various `Output::produce()` will create large future.
/// which is expensive to move around.
///
/// ```rust
/// // The size of this return future will be the largest size of `Output::produce(..)`
/// async fn execute(id, ...) { // in this case, it's `4kb`
///     match id {
///         1 => Output::produce(...).await, // Lets say it is create `64 kb` future
///         2 => Output::produce(...).await, // And this create `4kb` future
///         ...
///     }
/// }
/// ```
///
/// A solution is to wrap this function with `Box`.
/// But `transport.unary(..)` internally uses `Box` to wrap the future.
/// So let's take advantage of it.
enum FutState<'reader, 'de, Func, State, Args, Fut>
where
    Func: std_lib::FnOnce<Args, Output = Fut>,
{
    Init {
        func: Func,
        state: State,
        reader: &'reader mut &'de [u8],
        _args: std::marker::PhantomData<Args>,
    },
    Poll(Fut),
    Done,
}

impl<'reader, 'de, Func, State, Args, Fut> FutState<'reader, 'de, Func, State, Args, Fut>
where
    Func: std_lib::FnOnce<Args, Output = Fut>,
    Args: input::Input<'de, State>,
{
    fn new(func: Func, state: State, reader: &'reader mut &'de [u8]) -> Self {
        FutState::Init {
            func,
            state,
            reader,
            _args: std::marker::PhantomData,
        }
    }

    fn poll<T>(&mut self, cb: impl FnOnce(&mut Fut) -> Poll<io::Result<T>>) -> Poll<io::Result<T>> {
        loop {
            match self {
                FutState::Init { .. } => match std::mem::replace(self, FutState::Done) {
                    FutState::Init {
                        state,
                        reader,
                        func,
                        ..
                    } => match Args::decode(state, reader) {
                        // This is the only place where we move this future.
                        // From now on we promise we will never move it again!
                        Ok(args) => *self = FutState::Poll(func.call_once(args)),
                        Err(err) => {
                            return Poll::Ready(Err(io::Error::new(
                                io::ErrorKind::InvalidInput,
                                err,
                            )));
                        }
                    },
                    _ => unsafe { std::hint::unreachable_unchecked() },
                },
                FutState::Poll(ref mut fut) => return cb(fut),
                FutState::Done => panic!("`Output::produce` polled after completion"),
            }
        }
    }
}

impl<Fut> Output for Fut
where
    Fut: Future + Send,
    Fut::Output: Encode + TypeId,
{
    fn produce<'de, 'reader, 'transport, 'fut, State, Args>(
        func: impl 'fut + std_lib::FnOnce<Args, Output = Self> + Send,
        state: State,
        reader: &'reader mut &'de [u8],
        transport: &'transport mut (impl 'fut + Transport + Send),
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'fut>>
    where
        'de: 'fut,
        'reader: 'fut,
        'transport: 'fut,
        Self: 'fut,
        State: 'fut + Send,
        Args: 'fut + input::Input<'de, State> + Send,
    {
        let mut fut_state = FutState::new(func, state, reader);
        transport.unary(move |cx, buf| {
            fut_state.poll(|fut| {
                unsafe { Pin::new_unchecked(fut) }
                    .poll(cx)
                    .map(|data| Encode::encode::<{ crate::DATABUF_CONFIG }>(&data, buf))
            })
        })
    }
}

impl<G> Output for SSE<G>
where
    G: AsyncGenerator + Send,
{
    fn produce<'de, 'reader, 'transport, 'fut, State, Args>(
        func: impl 'fut + std_lib::FnOnce<Args, Output = Self> + Send,
        state: State,
        reader: &'reader mut &'de [u8],
        transport: &'transport mut (impl 'fut + Transport + Send),
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'fut>>
    where
        'de: 'fut,
        'reader: 'fut,
        'transport: 'fut,
        Self: 'fut,
        State: 'fut + Send,
        Args: 'fut + input::Input<'de, State> + Send,
    {
        let mut fut_state = FutState::new(func, state, reader);
        transport.server_stream(move |cx, buf| {
            fut_state.poll(|this| {
                unsafe { Pin::new_unchecked(&mut this.0) }
                    .poll_resume(cx)
                    .map(|gen_state| match gen_state {
                        GeneratorState::Yielded(val) => {
                            Encode::encode::<{ crate::DATABUF_CONFIG }>(&val, buf).map(|()| false)
                        }
                        GeneratorState::Complete(val) => {
                            Encode::encode::<{ crate::DATABUF_CONFIG }>(&val, buf).map(|()| true)
                        }
                    })
            })
        })
    }
}
