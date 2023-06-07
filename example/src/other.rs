// use std::collections::*;
// struct Collections {
//     vec: Vec<()>,
//     vec_deque: VecDeque<()>,
//     btree_map: BTreeMap<(), ()>,
//     btree_set: BTreeSet<()>,
//     hash_map: HashMap<(), ()>,
//     hash_ser: HashSet<(), ()>,
//     binery_heap: BinaryHeap<()>,
//     link_list: LinkedList<()>,
// }

// use databuf::{Decode, Encode};
// // #[derive(Message, Encode, Decode, Debug, PartialEq, Default)]
// struct Num {
//     u8: u8,
//     u16: u16,
//     u32: u32,
//     u64: u64,
//     u128: u128,
//     usize: usize,
// }

// // #[derive(Message, Encode, Decode, Debug, PartialEq, Default)]
// struct NegNum {
//     i8: i8,
//     i16: i16,
//     i32: i32,
//     i64: i64,
//     i128: i128,
//     isize: isize,
// }

// // #[derive(Message, Encode, Decode, Debug, PartialEq, Default)]
// struct FNum {
//     f32: f32,
//     f64: f64,
// }

// // #[derive(Message, Encode, Decode, Debug, PartialEq, Default)]
// enum Str<'a> {
//     Char(char),
//     Slice(&'a str),
//     Owned(String),
// }

// struct Data<'a> {
//     num: Num,
//     neg: NegNum,
//     fnum: FNum,
//     string: Str<'a>,
//     bool: bool,
// }

// #[derive(Message, Encode, Decode, Debug, PartialEq, Default)]
// struct EmptyTuple(((), ()), ());
