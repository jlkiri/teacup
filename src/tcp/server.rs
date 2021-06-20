use std::io::Result;
use std::{net::*, thread};

pub trait RequestHandler: FnOnce(TcpStream) -> Result<()> {}
impl<T> RequestHandler for T where T: FnOnce(TcpStream) -> Result<()> {}

pub struct TcpServer {
    listener: TcpListener,
}

impl TcpServer {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> Self {
        let listener = TcpListener::bind(addr).expect("Failed to bind a TCP socket to address.");
        Self { listener }
    }

    pub fn listen<F: RequestHandler + Send + Copy + 'static>(&self, handler: F) -> Result<()> {
        println!("TCP server is listening at {}", self.listener.local_addr()?);

        loop {
            let (mut stream, addr) = self.listener.accept()?;
            println!("Incoming connection from {}", addr);

            thread::spawn(move || {
                handler(stream).expect("Failed to handle connection.");
            });
        }
    }
}
