use std::{fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread};
use std::time::Duration;

pub fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // Iterate over connection attempts, panicking if any have an error
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "html/index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "html/index.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "html/not-found.html"),
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();

}