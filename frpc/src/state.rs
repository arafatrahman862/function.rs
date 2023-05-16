use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, Clone, Copy)]
pub struct State<T>(pub T);

impl<T> crate::input::FirstArg<'_, T> for State<T> {
    fn decode(state: T, _: &mut &[u8]) -> databuf::Result<Self> {
        Ok(State(state))
    }
}

// impl<S: 'static> crate::input::FirstArg<'_, Box<dyn std::any::Any>> for State<S> {
//     fn decode(state: Box<dyn std::any::Any>, _: &mut &[u8]) -> databuf::Result<Self> {
//         Ok(Self(*state.downcast::<S>().unwrap()))
//     }
// }

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
