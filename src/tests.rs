#[cfg(test)]
mod tests {
    use std::net::TcpStream;
    use std::io::{BufRead, BufReader, Write};
    use std::thread;
    use std::time::Duration;

    fn get_page(page: &str) -> (String, Duration) {
        let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
        let start = std::time::Instant::now();
        stream.write_all(format!("{page}\r\n\r\n").as_bytes()).unwrap();
        let mut reader = BufReader::new(stream);
        let mut response = String::new();
        reader.read_line(&mut response).unwrap();
        (response, start.elapsed())
    }

    /// This test is slow because it runs in parallel with the test that calls the sleep() method.
    /// The fact that these tests are not idempotent is a consequence of the server under test being
    /// single-threaded. When the server is multithreaded, theese tests will no longer interfere
    /// and it_loads_fast() will pass.
    #[test]
    fn it_loads_eventually() {
        let (response, duration) = get_page("GET /sleep HTTP/1.1");
        assert!(response.starts_with("HTTP/1.1 200 OK"));
        assert!(duration.as_secs() < 6, "Response took too long: {:?}", duration)
    }

    // #[test]
    // fn it_loads_fast() {
    //     let (response, duration) = get_page("GET /sleep HTTP/1.1");
    //     assert!(response.starts_with("HTTP/1.1 200 OK"));
    //     assert!(duration.as_millis() < 500, "Response took too long: {:?}", duration)
    // }

    #[test]
    fn it_loads_slow() {
        let (response, duration) = get_page("GET /sleep HTTP/1.1");
        assert!(response.starts_with("HTTP/1.1 200 OK"));
        assert!(duration.as_secs() >= 5, "Response was too fast: {:?}", duration)
    }
}