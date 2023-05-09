use std::{ops::Deref, sync::Arc};

#[derive(Debug, Default)]
pub struct Ctx<T: ?Sized> {
    inner: Arc<T>,
}

impl<T: ?Sized> Ctx<T> {
    #[inline]
    pub fn new(v: Arc<T>) -> Self {
        Self { inner: v }
    }
}

impl<ARC, T: ?Sized> From<ARC> for Ctx<T>
where
    ARC: Into<Arc<T>>,
{
    fn from(arc: ARC) -> Self {
        Self { inner: arc.into() }
    }
}

impl<T: ?Sized> Clone for Ctx<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T: ?Sized> Deref for Ctx<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
