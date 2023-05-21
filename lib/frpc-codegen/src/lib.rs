pub mod fmt;
pub mod javascript;
mod path;
pub mod utils;

use frpc_message::TypeDef;
pub use path::Path;

pub struct CodeGen<'a> {
    type_def: &'a TypeDef,
    input_paths: Vec<&'a String>,
    output_paths: Vec<&'a String>,
}

impl<'a> CodeGen<'a> {
    pub fn new(type_def: &'a TypeDef) -> Self {
        let mut input = Path::new(&type_def.costom_types);
        let mut output = Path::new(&type_def.costom_types);

        input.add_tys(type_def.funcs.iter().flat_map(|func| func.args.iter()));
        output.add_tys(type_def.funcs.iter().map(|func| &func.retn));

        Self {
            type_def,
            input_paths: input.paths,
            output_paths: output.paths,
        }
    }
}
