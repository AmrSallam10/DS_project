use std::net::UdpSocket;
extern crate serde;
use serde_derive::Serialize;

#[derive(Serialize)]
struct MSG {
    field1: u32,
    field2: String,
    field3: bool,
}

fn main() -> std::io::Result<()> {
    let msg = MSG {
        field1: 42,
        field2: String::from("Hello! This is CLIENT2"),
        field3: true,
    };

    let ips = vec!["127.0.0.1:8080", "127.0.0.1:8081"];
    let json = serde_json::to_string(&msg).unwrap();
    
    let socket = UdpSocket::bind("0.0.0.0:0")?; // Use an ephemeral port for the client

    for ip in ips {
        socket.send_to(json.as_bytes(), ip)?;

        let mut buffer = [0; 1024];
        let (bytes_read, _source) = socket.recv_from(&mut buffer)?;
        let response = std::str::from_utf8(&buffer[..bytes_read]).unwrap();
        println!("Server response: {}", response);
    }

    Ok(())
}
