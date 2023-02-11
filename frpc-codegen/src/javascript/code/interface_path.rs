use super::*;

#[derive(Debug)]
pub struct InterfacePath<'a> {
    ctx: &'a Context,
    pub paths: Vec<&'a String>,
}

impl<'a> InterfacePath<'a> {
    pub fn new(ctx: &'a Context) -> Self {
        Self { paths: vec![], ctx }
    }
}

impl<'a> InterfacePath<'a> {
    pub fn add_tys(&mut self, tys: impl Iterator<Item = &'a Ty>) {
        for ty in tys {
            self.add(ty)
        }
    }

    pub fn add(&mut self, ty: &'a Ty) {
        match ty {
            Ty::Map { ty, .. } => self.add(&ty.1),
            Ty::Result(ty) => {
                self.add(&ty.0);
                self.add(&ty.1);
            }
            Ty::Tuple(tys) => self.add_tys(tys.iter()),
            Ty::Option(ty) | Ty::Array { ty, .. } | Ty::Set { ty, .. } => self.add(ty),
            Ty::CustomType(path) if !self.paths.contains(&path) => {
                self.paths.push(path);
                match &self.ctx.costom_types[path] {
                    CustomTypeKind::Enum(data) => {
                        for data in data.fields.iter() {
                            match &data.kind {
                                EnumKind::Tuple(fields) => {
                                    self.add_tys(fields.iter().map(|f| &f.ty))
                                }
                                EnumKind::Struct(fields) => {
                                    self.add_tys(fields.iter().map(|f| &f.ty))
                                }
                                EnumKind::Unit => {}
                            }
                        }
                    }
                    CustomTypeKind::Tuple(data) => self.add_tys(data.fields.iter().map(|f| &f.ty)),
                    CustomTypeKind::Struct(data) => self.add_tys(data.fields.iter().map(|f| &f.ty)),
                    CustomTypeKind::Unit(_) => {}
                }
            }
            _ => {}
        }
    }
}
