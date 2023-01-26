use super::fmt;

pub struct Formatter<FmtFn> {
    func: FmtFn,
}

impl<FmtFn> Formatter<FmtFn>
where
    FmtFn: Fn(&mut std::fmt::Formatter) -> std::fmt::Result,
{
    #[inline]
    pub fn new(func: FmtFn) -> Self {
        Self { func }
    }
}

impl<FmtFn> std::fmt::Debug for Formatter<FmtFn>
where
    FmtFn: Fn(&mut std::fmt::Formatter) -> std::fmt::Result,
{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        (self.func)(f)
    }
}


pub fn ident(name: &String) -> fmt!() {
    fmt!(move |f| { f.write_fmt(format_args!("{}", &name)) })
}
