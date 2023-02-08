use core::fmt;

#[repr(transparent)]
pub struct Formatter<Fmt> {
    func: Fmt,
}

impl<Fmt> Formatter<Fmt>
where
    Fmt: Fn(&mut fmt::Formatter) -> fmt::Result,
{
    #[inline]
    pub fn new(func: Fmt) -> Self {
        Self { func }
    }
}

impl<Fmt> fmt::Debug for Formatter<Fmt>
where
    Fmt: Fn(&mut fmt::Formatter) -> fmt::Result,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.func)(f)
    }
}

impl<Fmt> fmt::Display for Formatter<Fmt>
where
    Fmt: Fn(&mut fmt::Formatter) -> fmt::Result,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.func)(f)
    }
}
