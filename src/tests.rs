#[cfg(test)]
mod tests {
    use std::net::TcpStream;
    use std::io::{BufRead, BufReader, Write};
    use std::thread;


    #[test]
    fn it_loads() {
        let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
        let start = std::time::Instant::now();
        stream.write_all("GET / HTTP/1.1\r\n\r\n".as_bytes()).unwrap();
        let mut reader = BufReader::new(stream);
        let mut response = String::new();
        reader.read_line(&mut response).unwrap();
        let duration = start.elapsed();
        assert!(response.starts_with("HTTP/1.1 200 OK"));
        assert!(duration.as_millis() < 500, "Response took too long: {:?}", duration)
    }
}