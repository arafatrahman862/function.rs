use super::*;
use std::{collections::*, hash::Hash};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetVariant {
    BTreeSet,
    HashSet,
    BinaryHeap,
    LinkedList,
    VecDeque,
    Vec,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapVariant {
    HashMap,
    BTreeMap,
}

macro_rules! impl_ty_class {
    [Set for $name: tt <$($ty_arg: ty),*> where $($ty: tt)*] => {
        impl<$($ty)*> Message for $name<$($ty_arg),*> {
            fn ty(def: &mut Definition) -> Type {
                Type::Set {
                    variant: SetVariant::$name,
                    ty: Box::new(T::ty(def)),
                }
            }
        }
    };
    [Map for $name: tt <$($ty_arg: ty),*> where $($ty: tt)*] => {
        impl<$($ty)*> Message for $name<$($ty_arg),*> {
            fn ty(def: &mut Definition) -> Type {
                Type::Map {
                    variant: MapVariant::$name,
                    ty: Box::new((K::ty(def), V::ty(def))),
                }
            }
        }
    };
}

impl_ty_class!(Set for Vec<T>             where T: Message);
impl_ty_class!(Set for VecDeque<T>        where T: Message);
impl_ty_class!(Set for LinkedList<T>      where T: Message);
impl_ty_class!(Set for BTreeSet<T>        where T: Message + Ord);
impl_ty_class!(Set for BinaryHeap<T>      where T: Message + Ord);
impl_ty_class!(Set for HashSet<T>         where T: Message + Eq + Hash);
impl_ty_class!(Map for BTreeMap<K, V>     where K: Message + Ord, V: Message);
impl_ty_class!(Map for HashMap<K, V>      where K: Message + Eq + Hash, V: Message);
