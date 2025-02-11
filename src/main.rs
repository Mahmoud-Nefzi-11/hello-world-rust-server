use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3033").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // Respond with a simple "Hello world"
    let status_line = "HTTP/1.1 200 OK";
    let contents = "Hello world";
    let length = contents.len();
    let content_type = "Content-Type: text/plain; charset=UTF-8";

    // Format the HTTP response
    let response = format!(
        "{status_line}\r\n{content_type}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    // Send the response to the client
    stream.write_all(response.as_bytes()).unwrap();
}
