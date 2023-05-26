use super::*;
use frpc_codegen::client::CodeGenerator;

pub fn init() {
    let codegen = CodeGenerator::from(Example);
}
