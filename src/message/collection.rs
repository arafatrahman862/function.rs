use std::{collections::*, hash::Hash};

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
