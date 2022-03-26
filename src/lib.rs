#![allow(warnings)]
#![feature(type_name_of_val)]
#![allow(non_camel_case_types)]

use std::any::*;
use std::fmt::Debug;

fn hello(num: i32) -> Box<dyn Any> {
    Box::new(format!("My lucky num: {}", num))
}

struct hello {}
impl hello {
    fn type_info() {
        println!("{:#?}", type_name::<i32>());
        println!("{:#?}", type_name_of_val(&hello(4)));

        println!("{:#?}", hello(4).as_ref().type_id());
        println!("{:#?}", TypeId::of::<std::string::String>());
    }
}

#[test]
fn test_name() {
    hello::type_info()
}

pub fn type_id_of_val<T: ?Sized + 'static>(_: &T) -> TypeId {
    TypeId::of::<T>()
}
