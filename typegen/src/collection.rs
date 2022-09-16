use super::*;
use std::collections::*;

#[derive(Debug)]
pub enum SetType {
    BTreeSet,
    HashSet,
    BinaryHeap,
    LinkedList,
    VecDeque,
    Vec,
}

#[derive(Debug)]
pub enum MapType {
    HashMap,
    BTreeMap,
}

macro_rules! impl_ty_class {
    [Set for $name: tt <$($ty_arg: ty),*> where $($ty: tt)*] => {
        impl<$($ty)*> GetType for $name<$($ty_arg),*> {
            #[inline] fn get_ty() -> Type {
                Type::Set {
                    collection_ty: SetType::$name,
                    ty: Box::new(T::get_ty()),
                }
            }
        }
    };
    [Map for $name: tt <$($ty_arg: ty),*> where $($ty: tt)*] => {
        impl<$($ty)*> GetType for $name<$($ty_arg),*> {
            #[inline] fn get_ty() -> Type {
                Type::Map {
                    collection_ty: MapType::$name,
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
