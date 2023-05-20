use core::fmt;

pub struct Fmt<F>(F);

impl<F> Fmt<F>
where
    F: Fn(&mut fmt::Formatter) -> fmt::Result,
{
    pub fn new(func: F) -> Self {
        Self(func)
    }
}

impl<F> fmt::Debug for Fmt<F>
where
    F: Fn(&mut fmt::Formatter) -> fmt::Result,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.0)(f)
    }
}

impl<F> fmt::Display for Fmt<F>
where
    F: Fn(&mut fmt::Formatter) -> fmt::Result,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.0)(f)
    }
}

#[macro_export]
macro_rules! fmt {
    (box $c: expr) => { $crate::fmt::Fmt::<Box<dyn Fn(&mut std::fmt::Formatter) -> std::fmt::Result>>::new(Box::new($c)) };
    ($c: expr) => { $crate::fmt::Fmt::new($c) };

    (type $lt: lifetime) => { $crate::fmt::Fmt<impl Fn(&mut std::fmt::Formatter) -> std::fmt::Result + $lt> };
    (type) => { $crate::fmt::Fmt<impl Fn(&mut std::fmt::Formatter) -> std::fmt::Result> };
}
