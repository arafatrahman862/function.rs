pub async fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub async fn print(msg: &str) {
    println!("{:?}", msg);
}
