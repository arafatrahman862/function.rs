use super::*;

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
        write!(f, "{}): ", fmt_args.join(", "))?;

        match &self.ret {
            Type::Tuple(tys) => match tys.is_empty() {
                true => write!(f, "void"),
                false => f.debug_list().entries(tys).finish(),
            },
            ty => write!(f, "{:?}", TypeOf(ty)),
        }
    }
}

#[test]
fn test() {
    let mut fn_ty = Func::new("foo");
    fn_ty
        .arg("a", Type::U8.optional())
        .arg("b", Type::U16)
        .arg("c", Type::U32.optional())
        .ret(Type::Any);

    assert_eq!(
        "function foo(a: number | undefined, b: number, c?: number): any",
        format!("{:?}", fn_ty)
    )
}
