use super::*;
use Ty;

#[derive(Default, Debug)]
pub(crate) struct InterfacePath<'a> {
    pub(crate) paths: Vec<&'a String>,
}

impl<'a> InterfacePath<'a> {
    pub(crate) fn add_tys(&mut self, tys: impl Iterator<Item = &'a Ty>, ctx: &'a Context) {
        tys.filter_map(|ty| match ty {
            Ty::CustomType(path) => Some(path),
            _ => None,
        })
        .for_each(|path| self.add(path, ctx));
    }

    pub(crate) fn add(&mut self, path: &'a String, ctx: &'a Context) {
        self.paths.push(path);
        match &ctx.costom_types[path] {
            CustomTypeKind::Enum(data) => {
                for data in data.fields.iter() {
                    match &data.kind {
                        EnumKind::Tuple(fields) => self.add_tys(fields.iter().map(|f| &f.ty), ctx),
                        EnumKind::Struct(fields) => self.add_tys(fields.iter().map(|f| &f.ty), ctx),
                        EnumKind::Unit => {}
                    }
                }
            }
            CustomTypeKind::Tuple(data) => {
                self.add_tys(data.fields.iter().map(|f| &f.ty), ctx);
            }
            CustomTypeKind::Struct(data) => {
                self.add_tys(data.fields.iter().map(|f| &f.ty), ctx);
            }
            CustomTypeKind::Unit(_) => {}
        }
    }
}
