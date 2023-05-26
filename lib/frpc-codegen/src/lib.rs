pub mod fmt;
pub mod javascript;
mod path;
pub mod utils;

use frpc_message::TypeDef;
use path::Path;

pub struct CodeGen {
    type_def: TypeDef,
    input_paths: Vec<String>,
    output_paths: Vec<String>,
}

impl From<TypeDef> for CodeGen {
    fn from(type_def: TypeDef) -> Self {
        let mut input = Path::new(&type_def.costom_types);
        let mut output = Path::new(&type_def.costom_types);

        input.add_tys(type_def.funcs.iter().flat_map(|func| func.args.iter()));
        output.add_tys(type_def.funcs.iter().map(|func| &func.retn));

        let input_paths = input.paths;
        let output_paths = output.paths;
        Self {
            type_def,
            input_paths,
            output_paths,
        }
    }
}
