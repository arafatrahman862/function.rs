use super::*;
use crate::typescript::interface::fmt_js_ty;

pub fn main(f: &mut impl Write, type_def: &TypeDef) -> Result {
    writeln!(f, "export default class Self {{")?;
    writeln!(f, "constructor(private rpc: use.RpcTransport) {{}}")?;
    writeln!(f, "static close(this: Self) {{ this.rpc.close() }}")?;

    type_def.funcs.iter().try_for_each(
        |Func {
             index,
             ident: path,
             args,
             retn,
         }| {
            let ident = path.replace("::", "_");

            write!(f, "async {ident}(")?;
            for (num, ty) in args.iter().enumerate() {
                write!(f, "_{num}: {}, ", fmt_js_ty(ty))?;
            }
            writeln!(f, ") {{")?;

            writeln!(f, "let fn = this.rpc.unary();")?;
            writeln!(f, "let d = new use.BufWriter(fn);")?;
            writeln!(f, "d.u16({index});")?;

            for (num, arg) in args.iter().enumerate() {
                match arg {
                    Ty::CustomType(path) => {
                        writeln!(f, "extern.{}(d, _{num});", object_ident_from(path))?
                    }
                    ty => writeln!(f, "{}(_{num});", fmt_ty(ty, "extern"))?,
                };
            }
            writeln!(f, "d.flush();")?;
            writeln!(f, "let _d = await fn.call();")?;
            if !retn.is_empty_tuple() {
                writeln!(f, "{{")?;
                writeln!(f, "let d = new use.Decoder(_d.buffer);")?;
                let res = match retn {
                    Ty::CustomType(path) => format!("struct.{}(d)", object_ident_from(path)),
                    ty => format!("{}()", fmt_ty(ty, "struct")),
                };
                writeln!(f, "return {res}")?;
                writeln!(f, "}}")?;
            }
            writeln!(f, "}}")
        },
    )?;
    writeln!(f, "}}")
}
