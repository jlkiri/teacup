use std::{
    io::{Read, Write},
    net::*,
    thread, vec,
};

const BUFFER_SIZE: usize = 32;

fn handle_connection(socket: UdpSocket) -> std::io::Result<()> {
    let mut buf = [0u8; BUFFER_SIZE];

    let (len, addr) = socket.recv_from(&mut buf)?;

    println!("Received a UDP packet from {}", addr);
    println!("Content: {}", String::from_utf8_lossy(&buf[..len]));

    Ok(())
}

pub struct UdpServer;
pub struct UdpClient;

impl UdpServer {
    pub fn listen<A: ToSocketAddrs>(addr: A) -> std::io::Result<()> {
        let socket = UdpSocket::bind(addr)?;
        handle_connection(socket)
    }
}

impl UdpClient {
    pub fn send<A: ToSocketAddrs>(addr: A, data: String) {
        let socket = UdpSocket::bind("127.0.0.1:9999").expect("Failed to bind.");
        socket
            .send_to(data.as_bytes(), addr)
            .expect("Failed to send UDP message.");
    }
}
