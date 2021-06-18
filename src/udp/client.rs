use std::net::*;

pub struct UdpClient;

impl UdpClient {
    pub fn send<A: ToSocketAddrs>(addr: A, data: String) {
        let socket = UdpSocket::bind("127.0.0.1:9999").expect("Failed to bind.");
        socket
            .send_to(data.as_bytes(), addr)
            .expect("Failed to send UDP message.");
    }
}
