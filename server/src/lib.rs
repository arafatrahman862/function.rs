#![allow(warnings)]

mod client;
pub mod proto;
pub mod utils;

use utils::*;

use std::{
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
};

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;
    println!("Listening on: {}", listener.local_addr()?);

    for stream in listener.incoming() {
        let mut stream = stream?;

        let mut buf = [0; 1024];
        let len = stream.read(&mut buf)?;
        let data = &buf[..len];

        if buf.starts_with(b"GET /echo") {
            stream.write_all(&data)?;
        }
        if buf.starts_with(b"GET /index") {
            stream.write_all(&html("../index.html"))?;
        }
        if buf.starts_with(b"GET /chat") {
            println!("{:#?}", sec_web_socket_key(&data));
            // println!("{}", String::from_utf8_lossy(&data));
        }
    }
    Ok(())
}


#[test]
fn test() {
    println!("{:?}", main());
}


fn websocket_conn_update() {}


