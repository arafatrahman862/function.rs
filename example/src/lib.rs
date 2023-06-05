async fn add(a: i32, b: i32) -> i32 {
    a + b
}

async fn print(msg: &str) {
    println!("{:?}", msg);
}

frpc::declare! {
    pub service Example {
        add = 1,
        print = 2,
    }
}
