use crate::{utils::to_camel_case, writer::Writer};
use frpc_message::*;
use std::fmt::{Result, Write};

#[allow(unused_macros)]
macro_rules! code {
    ($($tt:tt)*) => {};
}

code! {}

pub fn generate(w: &mut Writer, type_def: &TypeDef) -> Result {
    let interface = type_def.funcs.iter().filter_map(|func| match &func.retn {
        Ty::CustomType(path) => Some(path),
        _ => None,
    });

    for path in interface {
        let name = to_camel_case(path, ':');
        match &type_def.ctx.costom_types[path] {
            CustomTypeKind::Unit(data) => {
                fun_name(w, &name, data)?;
            }
            CustomTypeKind::Enum(_) => {}
            CustomTypeKind::Struct(_) => {}
            CustomTypeKind::TupleStruct(_) => {}
        }
    }
    Ok(())
}

fn fun_name(w: &mut Writer, name: &String, data: &CustomType<UnitField>) -> Result {
    writeln!(w, "function {name}(d) {}", '{')?;

    writeln!(w, "switch (d.len_u15()) {}", '{')?;

    for field in &data.fields {
        writeln!(w, "case {}: return {name}.{};", field.value, field.name)?;
    }

    writeln!(w, "{}", '}')?;
    writeln!(w, "{}", '}')?;
    Ok(())
}

#[test]
#[rustfmt::skip]
fn test_name() {
    let mut w = Writer::new();
    let fields = vec![
        UnitField { doc: "".into(), name: "Aa".into(), value: 1 },
        UnitField { doc: "".into(), name: "Zz".into(), value: 2 },
    ];
    fun_name(&mut w, &"Add".into(), &CustomType { doc: "".into(), fields }).unwrap();
    println!("{}", w.buf);
}
