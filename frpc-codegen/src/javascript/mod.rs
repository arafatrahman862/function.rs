pub mod generate;

pub mod interface;
use crate::Provider;
use crate::{fmt, utils::to_camel_case};
use interface::gen_type;

pub fn generate<'a>(provider: &'a Provider) -> fmt!(type 'a) {
    fmt!(move |f| {
        for path in provider.type_def.ctx.costom_types.keys() {
            let ident = to_camel_case(path, ':');
            gen_type(f, ident, &provider.type_def.ctx.costom_types[path])?;
        }
        generate::decoder::main(f, provider)?;
        generate::encoder::main(f, provider)?;
        generate::stub::main(f, provider.type_def)
    })
}
