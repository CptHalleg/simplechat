use std::io::Read;
use std::io::Write;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

use log::error;
use log::info;
use shared::constants::PORT;

fn handle_connection(mut stream: TcpStream, addr: SocketAddr) {
    info!("New Connection form {}", addr);
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size > 0 {
                    let message = String::from_utf8_lossy(&buffer[..size]);
                    info!("Message Recieved from {}:{}", addr, message);
                    stream.write_all(&buffer);
                } else {
                    info!("closing Connection with {}", addr);
                    return;
                }
            }
            Err(e) => {
                error!("Error with {}: {:#?}", addr, e);
            }
        }
    }
}

fn start_server() {
    info!("Starting server...");
    let ip = format!("127.0.0.1:{}", PORT);
    let listener = TcpListener::bind(&ip).expect("Failed to start Server");
    info!("Server running on {}", ip);
    for inc in listener.incoming() {
        match inc {
            Ok(stream) => match stream.peer_addr() {
                Ok(addr) => {
                    let a = addr.clone();
                    thread::spawn(move || handle_connection(stream, a));
                }
                Err(e) => error!("Unknown peer address {:#?}", e),
            },
            Err(e) => error!("Error {:#?}", e),
        }
    }
}

pub fn main() {
    env_logger::init();
    info!("Logger initialized!");
    start_server();
}
