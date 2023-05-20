use super::*;
use std::{collections::*, hash::Hash};

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "encode", derive(Encode))]
pub enum SetVariant {
    BTreeSet,
    HashSet,
    BinaryHeap,
    LinkedList,
    VecDeque,
    Vec,
}

#[cfg_attr(feature = "clone", derive(Clone))]
#[cfg_attr(feature = "hash", derive(Hash))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "encode", derive(Encode))]
pub enum MapVariant {
    HashMap,
    BTreeMap,
}

macro_rules! impl_ty_class {
    [Set for $name: tt <$($ty_arg: ty),*> where $($ty: tt)*] => {
        impl<$($ty)*> Message for $name<$($ty_arg),*> {
            fn ty(ctx: &mut Context) -> Ty {
                Ty::Set {
                    variant: SetVariant::$name,
                    ty: Box::new(T::ty(ctx)),
                }
            }
        }
    };
    [Map for $name: tt <$($ty_arg: ty),*> where $($ty: tt)*] => {
        impl<$($ty)*> Message for $name<$($ty_arg),*> {
            fn ty(ctx: &mut Context) -> Ty {
                Ty::Map {
                    variant: MapVariant::$name,
                    ty: Box::new((K::ty(ctx), V::ty(ctx))),
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

impl<T: Message> Message for &[T] {
    fn ty(ctx: &mut Context) -> Ty {
        Ty::Set {
            variant: SetVariant::Vec,
            ty: Box::new(T::ty(ctx)),
        }
    }
}
