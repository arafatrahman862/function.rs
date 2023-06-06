use super::*;
use crate::javascript::interface::fmt_js_ty;

pub fn main(f: &mut impl Write, type_def: &TypeDef) -> Result {
    writeln!(f, "export default class Self {{")?;
    writeln!(f, "constructor(private rpc: use.RpcTransport) {{}}")?;
    writeln!(f, "static close(this: Self) {{ this.rpc.close() }}")?;

    type_def.funcs.iter().try_for_each(
        |Func {
             index,
             path,
             args,
             retn,
         }| {
            let ident = path.replace("::", "_");

            write!(f, "{ident}(")?;
            for (num, ty) in args.iter().enumerate() {
                write!(f, "_{num}: {}, ", fmt_js_ty(ty))?;
            }
            writeln!(f, ") {{")?;

            writeln!(f, "const fn = this.rpc.unary()")?;
            writeln!(f, "const d = new use.BufWriter(fn);")?;
            writeln!(f, "d.u16({index});")?;

            for (num, arg) in args.iter().enumerate() {
                match arg {
                    Ty::CustomType(path) => {
                        writeln!(f, "extern.{}(d, _{num});", to_camel_case(path, ':'))?
                    }
                    ty => writeln!(f, "{}(_{num});", fmt_ty(ty, "extern"))?,
                };
            }
            writeln!(f, "d.flush();")?;

            if !retn.is_empty_tuple() {
                writeln!(f, "return fn.call().then(buf => new use.Decoder(buf))")?;
                let res = match retn {
                    Ty::CustomType(path) => format!("struct.{}", to_camel_case(path, ':')),
                    ty => format!("d => {}()", fmt_ty(ty, "struct")),
                };
                writeln!(f, ".then({res});")?;
            }
            writeln!(f, "}}")
        },
    )?;
    writeln!(f, "}}")
}
