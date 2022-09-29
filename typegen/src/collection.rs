use super::*;
use std::collections::*;

#[derive(Debug, Clone)]
pub enum SetVariant {
    BTreeSet,
    HashSet,
    BinaryHeap,
    LinkedList,
    VecDeque,
    Vec,
}

#[derive(Debug, Clone)]
pub enum MapVariant {
    HashMap,
    BTreeMap,
}

macro_rules! impl_ty_class {
    [Set for $name: tt <$($ty_arg: ty),*> where $($ty: tt)*] => {
        impl<$($ty)*> GetType for $name<$($ty_arg),*> {
            #[inline] fn get_ty() -> Type {
                Type::Set {
                    variant: SetVariant::$name,
                    ty: Box::new(T::get_ty()),
                }
            }
        }
    };
    [Map for $name: tt <$($ty_arg: ty),*> where $($ty: tt)*] => {
        impl<$($ty)*> GetType for $name<$($ty_arg),*> {
            #[inline] fn get_ty() -> Type {
                Type::Map {
                    variant: MapVariant::$name,
                    ty: Box::new((K::get_ty(), V::get_ty())),
                }
            }
        }
    };
}
impl_ty_class!(Set for Vec<T>             where T: GetType);
impl_ty_class!(Set for VecDeque<T>        where T: GetType);
impl_ty_class!(Set for LinkedList<T>      where T: GetType);
impl_ty_class!(Set for BinaryHeap<T>      where T: GetType);
impl_ty_class!(Set for BTreeSet<T>        where T: GetType);
impl_ty_class!(Set for HashSet<T, S>      where T: GetType, S);
impl_ty_class!(Map for BTreeMap<K, V>     where K: GetType, V: GetType);
impl_ty_class!(Map for HashMap<K, V, S>   where K: GetType, V: GetType, S);

impl SetVariant {
    pub fn ty_id(&self) -> u8 {
        match self {
            SetVariant::BTreeSet => 0,
            SetVariant::HashSet => 1,
            SetVariant::BinaryHeap => 2,
            SetVariant::LinkedList => 3,
            SetVariant::VecDeque => 4,
            SetVariant::Vec => 5,
        }
    }
}

impl TryFrom<u8> for SetVariant {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => SetVariant::BTreeSet,
            1 => SetVariant::HashSet,
            2 => SetVariant::BinaryHeap,
            3 => SetVariant::LinkedList,
            4 => SetVariant::VecDeque,
            5 => SetVariant::Vec,
            id => {
                return Err(format!(
                    "Can't create `{}` from `u8`: {id}",
                    type_name::<Self>()
                ))
            }
        })
    }
}

impl MapVariant {
    pub fn ty_id(&self) -> u8 {
        match self {
            MapVariant::HashMap => 0,
            MapVariant::BTreeMap => 1,
        }
    }
}
impl TryFrom<u8> for MapVariant {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => MapVariant::HashMap,
            1 => MapVariant::BTreeMap,
            id => {
                return Err(format!(
                    "Can't create `{}` from `u8`: {id}",
                    type_name::<Self>()
                ))
            }
        })
    }
}
