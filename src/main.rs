#![allow(warnings)]
mod utils;
use websocket;

use std::{
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
};
use utils::*;

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
            println!("{:#?}", websocket::utils::sec_web_socket_key(&data));
        }
    }
    Ok(())
}

#[test]
fn test_name() {
    let contents = [
        0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e,
        0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d,
        0x3e, 0x3f,
    ];
    std::fs::write("client-ephemeral-private.key", contents);
}
