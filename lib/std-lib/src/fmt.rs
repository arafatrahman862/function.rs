pub type BoxedFmt<'lt> = Fmt<Box<dyn Fn(&mut std::fmt::Formatter) -> std::fmt::Result + 'lt>>;

pub struct Fmt<F>(pub F)
where
    F: Fn(&mut std::fmt::Formatter) -> std::fmt::Result;

impl<F> std::fmt::Display for Fmt<F>
where
    F: Fn(&mut std::fmt::Formatter) -> std::fmt::Result,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        (self.0)(f)
    }
}

impl<F> std::fmt::Debug for Fmt<F>
where
    F: Fn(&mut std::fmt::Formatter) -> std::fmt::Result,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        (self.0)(f)
    }
}

#[macro_export]
macro_rules! fmt {
    (type $lt: lifetime) => { $crate::fmt::Fmt<impl Fn(&mut std::fmt::Formatter) -> std::fmt::Result + $lt> };
    (type) => { $crate::fmt::Fmt<impl Fn(&mut std::fmt::Formatter) -> std::fmt::Result> };
}

pub use fmt;
