use std::io::{self, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

use bin_layout::{Decoder, Encoder};

pub struct Client<RW> {
    is_open: bool,
    stream: RW,
}

impl<RW: Read + Write> Client<RW> {
    pub fn new(stream: RW) -> Self {
        Self {
            is_open: false,
            stream,
        }
    }

    fn call<Args, Ret>(&mut self, id: u16, data: Args) -> io::Result<Ret>
    where
        Args: Encoder,
        Ret: for<'de> Decoder<'de>,
    {
        let mut buf = Vec::new();
        (id, data).encoder(&mut buf)?;
        self.stream.write_all(&buf)?;

        let mut buf = [0; 3];
        self.stream.read_exact(&mut buf)?;
        let [b0, b1, b2] = buf;
        let len: usize = u32::from_le_bytes([b0, b1, b2, 0]).try_into().unwrap();

        let mut data = vec![0; len];
        self.stream.read_exact(&mut data)?;

        match Result::<Ret, String>::decode(&data).unwrap() {
            Ok(data) => Ok(data),
            Err(msg) => Err(io::Error::new(io::ErrorKind::ConnectionAborted, msg)),
        }
    }
}