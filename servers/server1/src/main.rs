extern crate serde;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
struct MSG {
    field1: u32,
    field2: String,
    field3: bool,
}

fn handle_request(buffer: &[u8], src_addr: std::net::SocketAddr) {
    let request = std::str::from_utf8(buffer).expect("Failed to convert to UTF-8");
    let msg: Result<MSG> = serde_json::from_str(request);

    match msg {
        Ok(msg) => {
            // Successfully deserialized JSON, you can use the 'msg' variable here
            println!("Received request from {}: {}", src_addr, msg.field2);
        }
        Err(err) => {
            eprintln!("Failed to deserialize JSON: {}", err);
            // Handle the error here, e.g., return an error response or take appropriate action.
        }
    }
    thread::sleep(Duration::from_secs(5))
}

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:8080");

    let mut buffer = [0; 1024];

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((bytes_read, src_addr)) => {
                handle_request(&buffer[..bytes_read], src_addr);

                let response = "Hello! This is SERVER1".as_bytes();
                socket.send_to(response, src_addr).expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
            }
        }
    }
}
