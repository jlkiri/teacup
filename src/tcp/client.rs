use crate::connection;
use std::io::Result;
use std::{
    net::{TcpStream, ToSocketAddrs},
};

pub struct TcpClient;

impl TcpClient {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<()> {
        let stream = TcpStream::connect(addr)?;
        connection::handle_connection(stream)
    }
}
