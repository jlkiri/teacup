use std::net::*;

const BUFFER_SIZE: usize = 32;

fn handle_connection(socket: UdpSocket) -> std::io::Result<()> {
    let mut buf = [0u8; BUFFER_SIZE];

    let (len, addr) = socket.recv_from(&mut buf)?;

    println!("Received a UDP packet from {}", addr);
    println!("Message: {}", String::from_utf8_lossy(&buf[..len]));

    Ok(())
}

pub struct UdpServer;

impl UdpServer {
    pub fn listen<A: ToSocketAddrs>(addr: A) -> std::io::Result<()> {
        let socket = UdpSocket::bind(addr)?;
        handle_connection(socket)
    }
}
