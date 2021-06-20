use std::net::*;

const BUFFER_SIZE: usize = 32;

fn handle_connection(socket: UdpSocket) {
    let mut buf = [0u8; BUFFER_SIZE];

    let (len, addr) = socket
        .recv_from(&mut buf)
        .expect("Failed to receive a UDP message.");

    println!("Received {} bytes from {}", len, addr);
    println!("Message: {}", String::from_utf8_lossy(&buf[..len]));
}

pub struct UdpServer;

impl UdpServer {
    pub fn listen<A: ToSocketAddrs>(addr: A) {
        let socket = UdpSocket::bind(addr).expect("Failed to bind UDP socket to address.");
        println!(
            "UDP socket is listening at {}",
            socket.local_addr().unwrap()
        );
        handle_connection(socket)
    }
}
