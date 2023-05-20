pub mod generate;
pub mod interface;
use crate::CodeGen;
use crate::{fmt, utils::to_camel_case};
use interface::gen_type;

impl<'a> CodeGen<'a> {
    pub fn javascript(&self) -> fmt!(type '_) {
        fmt!(move |f| {
            for path in self.type_def.ctx.costom_types.keys() {
                let ident = to_camel_case(path, ':');
                gen_type(f, ident, &self.type_def.ctx.costom_types[path])?;
            }
            generate::decoder::main(f, self)?;
            generate::encoder::main(f, self)?;
            generate::stub::main(f, self.type_def)
        })
    }
}
