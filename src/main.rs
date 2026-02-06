use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs::File;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let response = router(String::from_utf8_lossy(&buffer).parse().unwrap());

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn router(request: String) -> String {
    if target("/", &request) {
        let contents = html("hello.html");

        http_response(200, "OK", contents)
    }
    else {
        let contents = html("404.html");

        http_response(404, "NOT FOUND", contents)
    }
}

fn target(target: &str, request: &String) -> bool {
    let temp = format!("GET {} HTTP/1.1\r\n", target);
    request.starts_with(temp.as_str())
}

fn html(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}

fn http_response(response_code: u16, response_message: &str, content: String) -> String {
    format!("HTTP/1.1 {} {}\r\n\r\n{}", response_code, response_message, content)
}