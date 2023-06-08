use databuf::{Decode, Encode};
use frpc_macros::Message;

type DataType<'a> = (
    ((), ((), ())),
    (Option<Option<&'a str>>, Option<Option<String>>),
    (Result<i32, &'a str>, Result<i32, &'a str>),
    r#class,
    r#enum,
);

fn data() -> DataType<'static> {
    (
        // Empty typles
        ((), ((), ())),
        // Option
        (Some(Some("Some Data")), Some(None)),
        // Result
        (
            Result::<_, &str>::Ok(42),
            Result::<i32, _>::Err("Invalid Number!"),
        ),
        r#class { r#new: () },
        r#enum::r#type,
    )
}

async fn r#get_data() -> DataType<'static> {
    data()
}

async fn validate<'a>(_data: DataType<'a>) {
    println!("Result: {}", _data == data());
}

#[allow(non_camel_case_types)]
#[derive(Message, Encode, Decode, PartialEq)]
struct r#class {
    r#new: (),
}

#[allow(non_camel_case_types)]
#[derive(Message, Encode, Decode, PartialEq)]
enum r#enum {
    r#type,
}

frpc::declare! {
    pub service r#ValidateTest {
        r#get_data = 1,
        validate = 2,
    }
}
