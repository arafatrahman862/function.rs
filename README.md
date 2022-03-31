Which features a web framework should have ? Comments your favorite features below

What if there is no features! 

1. No Router!
2. No Middleware!
3. No Query Parser!
4. No MVC nonsense!
5. No Client API! (Rest, GraphQL)

There should be only one things, Just `Function`!  

Most modern languages has some sort of type information... So Let's create a tool that take those type definition and generate client side code!!!

Those are called `RPC`, Such tools already exist! (For example `tRPC`)   



```rust
fn hello() -> String {
    "Hello World".to_string()
}

ctx! {
   DB {
       hello,
       world,
   }
   
}
```

