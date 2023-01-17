use crate::*;
use std::{collections::*, hash::Hash};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetVariant {
    BTreeSet,
    HashSet,
    BinaryHeap,
    LinkedList,
    VecDeque,
    Vec,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MapVariant {
    HashMap,
    BTreeMap,
}

macro_rules! impl_ty_class {
    [Set for $name: tt <$($ty_arg: ty),*> where $($ty: tt)*] => {
        impl<$($ty)*> GetType for $name<$($ty_arg),*> {
            fn ty() -> Type {
                Type::Set {
                    variant: SetVariant::$name,
                    ty: Box::new(T::ty()),
                }
            }
        }
    };
    [Map for $name: tt <$($ty_arg: ty),*> where $($ty: tt)*] => {
        impl<$($ty)*> GetType for $name<$($ty_arg),*> {
            fn ty() -> Type {
                Type::Map {
                    variant: MapVariant::$name,
                    ty: Box::new((K::ty(), V::ty())),
                }
            }
        }
    };
}

impl_ty_class!(Set for Vec<T>             where T: GetType);
impl_ty_class!(Set for VecDeque<T>        where T: GetType);
impl_ty_class!(Set for LinkedList<T>      where T: GetType);
impl_ty_class!(Set for BTreeSet<T>        where T: GetType);
impl_ty_class!(Set for BinaryHeap<T>      where T: GetType);
impl_ty_class!(Set for HashSet<T, S>      where T: GetType, S);
impl_ty_class!(Map for BTreeMap<K, V>     where K: GetType, V: GetType);
impl_ty_class!(Map for HashMap<K, V, S>   where K: GetType, V: GetType, S);

impl<T: Resource> Resource for Vec<T> {}
impl<T: Resource> Resource for VecDeque<T> {}
impl<T: Resource> Resource for LinkedList<T> {}
impl<T: Resource + Ord> Resource for BTreeSet<T> {}
impl<T: Resource + Ord> Resource for BinaryHeap<T> {}
impl<T: Resource + Eq + Hash> Resource for HashSet<T> {}
impl<K: Resource + Ord, V: Resource> Resource for BTreeMap<K, V> {}
impl<K: Resource + Eq + Hash, V: Resource> Resource for HashMap<K, V> {}
