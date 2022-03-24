use super::*;

impl Func {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            args: vec![],
            ret: Type::Null,
        }
    }
    pub fn arg(&mut self, name: impl Into<String>, ty: Type) -> &mut Self {
        self.args.push(Field {
            name: name.into(),
            ty,
        });
        self
    }
    pub fn ret(&mut self, ty: Type) -> &mut Self {
        self.ret = ty;
        self
    }
}

impl Debug for Func {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "function {}(", self.name)?;

        let mut is_optional = true;
        let mut fmt_args: Vec<_> = self
            .args
            .iter()
            .rev()
            .map(|arg| match &arg.ty {
                Type::Option(ty) if is_optional == true => {
                    format!("{}?: {:?}", arg.name, TypeOf(ty))
                }
                ty => {
                    is_optional = false;
                    format!("{}: {:?}", arg.name, TypeOf(ty))
                }
            })
            .collect();

        fmt_args.reverse();
        write!(f, "{})", fmt_args.join(", "))?;

        match &self.ret {
            Type::Null => write!(f, ": void"),
            ty => write!(f, ": {:?}", TypeOf(ty)),
        }
    }
}

#[test]
fn test() {
    let mut fn_ty = Func::new("foo");
    fn_ty
        .arg("a", Type::Option(Box::new(Type::U8)))
        .arg("b", Type::U16)
        .arg("c", Type::Option(Box::new(Type::U32)))
        .ret(Type::Any);

    assert_eq!(
        "function foo(a: number | undefined, b: number, c?: number): any",
        format!("{:?}", fn_ty)
    )
}
