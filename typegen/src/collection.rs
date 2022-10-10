use super::*;
use std::collections::*;

#[derive(Debug, Clone, PartialEq)]
pub enum SetVariant {
    BTreeSet,
    HashSet,
    BinaryHeap,
    LinkedList,
    VecDeque,
    Vec,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MapVariant {
    HashMap,
    BTreeMap,
}

macro_rules! impl_ty_class {
    [Set for $name: tt <$($ty_arg: ty),*> where $($ty: tt)*] => {
        impl<$($ty)*> GetType for $name<$($ty_arg),*> {
            const TYPE: Type = Type::Set {
                variant: SetVariant::$name,
                ty: &T::TYPE
            };
        }
    };
    [Map for $name: tt <$($ty_arg: ty),*> where $($ty: tt)*] => {
        impl<$($ty)*> GetType for $name<$($ty_arg),*> {
            const TYPE: Type = Type::Map {
                variant: MapVariant::$name,
                ty: &(K::TYPE, V::TYPE)
            };
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
