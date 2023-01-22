use std::  {
    fs,
    io::{prelude::*, BufReader},
    net::TcpStream,
};

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("Hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\n]Content-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}