use std::{net::*, thread};

type IoResult<T> = std::io::Result<T>;

pub trait RequestHandler: FnOnce(&mut TcpStream) -> IoResult<()> {}
impl<T> RequestHandler for T where T: FnOnce(&mut TcpStream) -> IoResult<()> {}

pub struct TcpServer<A: ToSocketAddrs> {
    addr: A,
}

impl<A: ToSocketAddrs> TcpServer<A> {
    pub fn bind(addr: A) -> Self {
        Self { addr }
    }

    pub fn listen<F: RequestHandler + Send + Copy + 'static>(
        &self,
        handler: F,
    ) -> IoResult<()> {
        let listener = TcpListener::bind(&self.addr)?;

        println!("TCP server is listening at {}", listener.local_addr()?);

        loop {
            let (mut stream, addr) = listener.accept()?;
            println!("Incoming connection from {}", addr);

            thread::spawn(move || {
                handler(&mut stream).expect("Failed to handle connection.");
            });
        }
    }
}
