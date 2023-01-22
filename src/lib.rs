use std::  {
    fs,
    io::{prelude::*, BufReader},
    net::TcpStream,
};

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let response;

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("Hello.html").unwrap();
        let length = contents.len();

        response = format!("{status_line}\r\n]Content-Length: {length}\r\n\r\n{contents}");
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        response = format!("{status_line}\r\n]Content-Length: {length}\r\n\r\n{contents}");
    }

    stream.write_all(response.as_bytes()).unwrap();
}