#![allow(warnings)]
#![allow(non_camel_case_types)]

fn hello(s: String, n: i32) -> String {
    format!("Hello {} {}", s, n)
}

struct hello {}
impl hello {
    fn type_info() -> &'static str {
        std::any::type_name::<String>()
    }
}


#[test]
fn test_name() {
    println!("{:#?}", hello::type_info());
}