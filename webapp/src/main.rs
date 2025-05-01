#![allow(warnings)]

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    time::SystemTime,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("[INFO] Server listening on http://127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let peer_addr = stream.peer_addr().unwrap_or_else(|_| "Unknown".parse().unwrap());
                println!(
                    "[INFO] Connection established with {} at {:?}",
                    peer_addr,
                    SystemTime::now()
                );
                handle_connection(stream);
            }
            Err(e) => {
                eprintln!("[ERROR] Failed to establish connection: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap_or_else(|_| "Unknown".parse().unwrap());
    let buf_reader = BufReader::new(&stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap_or_else(|e| {
            eprintln!("[WARN] Error reading from stream: {}", e);
            String::new()
        }))
        .take_while(|line| !line.is_empty())
        .collect();

    println!("[DEBUG] Incoming request from {}:", peer_addr);
    for line in &http_request {
        println!("> {}", line);
    }

    let status_line = "HTTP/1.1 200 OK";

    let contents = match fs::read_to_string("hello.html") {
        Ok(data) => data,
        Err(err) => {
            eprintln!("[ERROR] Failed to read hello.html: {}", err);
            let error_message = "<h1>500 Internal Server Error</h1>";
            let response = format!(
                "HTTP/1.1 500 Internal Server Error\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
                error_message.len(),
                error_message
            );
            stream.write_all(response.as_bytes()).unwrap();
            return;
        }
    };

    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\nContent-Type: text/html\r\n\r\n{contents}"
    );

    println!("[INFO] Sending response to {}", peer_addr);
    stream.write_all(response.as_bytes()).unwrap();
}
