use std::net::{TcpListener, TcpStream};
use std::io::{Read};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    while match stream.read(&mut buffer) {
        Ok(size) if size > 0 => {
            println!("📩 Received: {}", String::from_utf8_lossy(&buffer[..size]));
            true
        },
        _ => false,
    } {}
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("⚠️ Failed to bind to port 7878");

    println!("📡 Groundstation is running...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => eprintln!("⚠️ Connection failed: {}", e),
        }
    }
}
