pub fn html(path: &str) -> Vec<u8> {
    let file = std::fs::read("../index.html").unwrap();

    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        file.len(),
        String::from_utf8(file).unwrap()
    )
    .into()
}
