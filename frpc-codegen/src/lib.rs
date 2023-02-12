pub mod javascript;

pub mod fmt;
pub mod utils;

#[macro_export]
macro_rules! fmt {
    (box $c: expr) => { crate::fmt::Fmt::<Box<dyn Fn(&mut std::fmt::Formatter) -> std::fmt::Result>>::new(Box::new($c)) };
    ($c: expr) => { crate::fmt::Fmt::new($c) };

    (type $lt: lifetime) => { crate::fmt::Fmt<impl Fn(&mut std::fmt::Formatter) -> std::fmt::Result + $lt> };
    (type) => { crate::fmt::Fmt<impl Fn(&mut std::fmt::Formatter) -> std::fmt::Result> };
}

// --------------------------------------------------------------

