pub mod generate;
pub mod interface;
use crate::CodeGen;
use crate::{fmt, utils::to_camel_case, Fmt};

impl CodeGen {
    pub fn typescript(&self) -> fmt!(type '_) {
        Fmt(move |f| {
            for (path, value) in &self.type_def.costom_types {
                interface::gen_type(f, to_camel_case(path, ':'), value)?;
            }
            generate::decoder::main(f, self)?;
            generate::encoder::main(f, self)?;
            generate::stub::main(f, &self.type_def)
        })
    }
}
