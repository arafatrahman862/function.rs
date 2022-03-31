pub fn html(path: &str) -> Vec<u8> {
    let file = std::fs::read("../index.html").unwrap();
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        file.len(),
        String::from_utf8(file).unwrap()
    )
    .as_bytes()
    .to_owned()
}

/// # Client handshake request
/// 
/// A client sends a handshake request to the server. It includes the following information:
/// 
/// ```yml
/// GET /chat HTTP/1.1
/// Host: example.com:8000
/// Upgrade: websocket
/// Connection: Upgrade
/// Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==
/// Sec-WebSocket-Version: 13
/// ```
/// 
/// ### Note
/// 
/// - The server must be careful to understand everything the client asks for, otherwise security issues can occur.
/// - If any header is not understood or has an incorrect value, the server should send a 400 ("Bad Request")} response and immediately close the socket.
/// - HTTP version must be `1.1` or greater, and method must be `GET`
/// - If the server doesn't understand that version of WebSockets, it should send a `Sec-WebSocket-Version` header back that contains the version(s) it does understand.
/// - All browsers send an Origin header.
///   You can use this header for security (checking for same origin, automatically allowing or denying, etc.) and send a 403 Forbidden if you don't like what you see.
///   However, be warned that non-browser agents can send a faked Origin. Most applications reject requests without this header.
pub fn sec_web_socket_key(bytes: &[u8]) -> Option<&str> {
    std::str::from_utf8(bytes)
        .ok()?
        .lines()
        .find(|s| s.starts_with("Sec-WebSocket-Key"))?
        .split(": ")
        .nth(1)
}