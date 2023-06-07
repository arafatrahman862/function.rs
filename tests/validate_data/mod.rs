use databuf::{Decode, Encode};
use frpc_macros::Message;

type DataType<'a> = (
    (
        (u8, u16, u32, u64, u128, usize),
        (u8, u16, u32, u64, u128, usize),
        (i8, i16, i32, i64, i128, isize),
        (i8, i16, i32, i64, i128, isize),
        (f32, f64),
        (f32, f64),
    ),
    (bool, bool),
    (String, &'a str, char),
    ((), ((), ())),
    (Option<Option<&'a str>>, Option<Option<String>>),
    (Result<i32, &'a str>, Result<i32, &'a str>),
);

fn data() -> DataType<'static> {
    (
        // Number
        (
            // MIN number
            (u8::MIN, u16::MIN, u32::MIN, u64::MIN, u128::MIN, usize::MIN),
            // MAX number
            (u8::MAX, u16::MAX, u32::MAX, u64::MAX, u128::MAX, usize::MAX),
            // Neg MIN number
            (i8::MIN, i16::MIN, i32::MIN, i64::MIN, i128::MIN, isize::MIN),
            // Neg MAX nimber
            (i8::MAX, i16::MAX, i32::MAX, i64::MAX, i128::MAX, isize::MAX),
            // MIN Flote
            (f32::MIN, f64::MIN),
            // MAX Flote
            (f32::MAX, f64::MAX),
        ),
        // Boolean
        (true, false),
        // String
        (String::from("Hello"), "World", '!'),
        // Empty typles
        ((), ((), ())),
        // Option
        (Some(Some("Some Data")), Some(None)),
        // Result
        (
            Result::<_, &str>::Ok(42),
            Result::<i32, _>::Err("Invalid Number!"),
        ),
    )
}

async fn get_data() -> DataType<'static> {
    data()
}

async fn validate<'a>(_data: DataType<'a>) {
    println!("Result: {}", _data == data());
}

#[derive(Message, Encode, Decode, Debug, Default)]
struct User {
    name: String,
    age: u8,
}
async fn user() -> User {
    User::default()
}

frpc::declare! {
    pub service ValidateData {
        get_data = 1,
        validate = 2,
        user = 3
    }
}
