use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, Clone, Copy)]
struct State<T>(pub T);

impl<S> crate::input::FirstArg<'_, S> for State<S> {
    fn decode(state: S, _: &mut &[u8]) -> databuf::Result<Self> {
        Ok(Self(state))
    }
}

impl<T> Deref for State<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for State<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
