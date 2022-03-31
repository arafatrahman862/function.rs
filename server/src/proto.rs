//! ### Keeping track of clients
//! 
//! This doesn't directly relate to the WebSocket protocol,
//! but it's worth mentioning here: your server must keep track of clients' sockets so you don't keep handshaking again with clients who have already completed the handshake.
//! The same client IP address can try to connect multiple times.
//! However, the server can deny them if they attempt too many connections in order to save itself from [Denial-of-Service attack](https://en.wikipedia.org/wiki/Denial-of-service_attack).
//!
//!
//! For example, you might keep a table of usernames or ID numbers along with the corresponding WebSocket and other data that you need to associate with that connection.
//!
//! ### Data frames
//! 
//! ```
//!      0                   1                   2                   3
//!      0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//!     +-+-+-+-+-------+-+-------------+-------------------------------+
//!     |F|R|R|R| opcode|M| Payload len |    Extended payload length    |
//!     |I|S|S|S|  (4)  |A|     (7)     |             (16/64)           |
//!     |N|V|V|V|       |S|             |   (if payload len==126/127)   |
//!     | |1|2|3|       |K|             |                               |
//!     +-+-+-+-+-------+-+-------------+ - - - - - - - - - - - - - - - +
//!     |     Extended payload length continued, if payload len == 127  |
//!     + - - - - - - - - - - - - - - - +-------------------------------+
//!     |                               |Masking-key, if MASK set to 1  |
//!     +-------------------------------+-------------------------------+
//!     | Masking-key (continued)       |          Payload Data         |
//!     +-------------------------------- - - - - - - - - - - - - - - - +
//!     :                     Payload Data continued ...                :
//!     + - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - +
//!     |                     Payload Data continued ...                |
//!     +---------------------------------------------------------------+
//! ```



/// ## Server handshake response
/// When the server receives the handshake request,
/// It should send back a special response that indicates that the protocol will be changing from HTTP to WebSocket.
///
/// The Sec-WebSocket-Accept header is important in that the server must derive it from the `Sec-WebSocket-Key` that the client sent to it.
/// 
/// ### Example
///
/// ```rust
/// let res = [
///     "HTTP/1.1 101 Switching Protocols",
///     "Upgrade: websocket",
///     "Connection: Upgrade",
///     "Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=",
///     "",
///     ""
/// ];
/// assert_eq!(handshake_res("dGhlIHNhbXBsZSBub25jZQ=="), res.join("\r\n"));
/// ```
///
/// To get it, concatenate the client's `Sec-WebSocket-Key` and the string _"258EAFA5-E914-47DA-95CA-C5AB0DC85B11"_ together (it's a [Magic string](https://en.wikipedia.org/wiki/Magic_string)), take the SHA-1 hash of the result, and return the base64 encoding of that hash.
///
/// ### Note
/// - Regular HTTP status codes can be used only before the handshake. After the handshake succeeds, you have to use a different set of codes (defined in section 7.4 of the spec)
pub fn handshake_res(sec_web_socket_key: &str) -> String {
    use sha1::{Digest, Sha1};
    let mut m = Sha1::new();
    m.update(sec_web_socket_key.as_bytes());
    m.update(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11"); // Magic string
    let key = base64::encode(m.finalize());

    format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {key}\r\n\r\n",)
}
