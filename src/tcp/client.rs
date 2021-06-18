use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

const BUFFER_SIZE: usize = 32;

pub struct TcpClient<A: ToSocketAddrs>(A);

impl<A: ToSocketAddrs> TcpClient<A> {
    pub fn new(addr: A) -> Self {
        Self(addr)
    }

    pub fn connect(&self) -> std::io::Result<()> {
        let stream = TcpStream::connect(&self.0)?;
        self.handle_stream(stream)
    }

    fn handle_stream(&self, mut stream: TcpStream) -> std::io::Result<()> {
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            stream.write_all(input.as_bytes())?;
            stream.flush()?;

            let mut buf = [0u8; BUFFER_SIZE];

            stream
                .set_read_timeout(Some(Duration::from_millis(100)))
                .unwrap();

            loop {
                match stream.read(&mut buf) {
                    Ok(0) => break,
                    Ok(len) => {
                        let mut received: Vec<u8> = vec![];
                        received.extend_from_slice(&buf[..len]);
                        println!(
                            "Server response: {}",
                            String::from_utf8(received).expect("Invalid utf-8")
                        );
                    }
                    Err(e) => {
                        if e.kind() != std::io::ErrorKind::Interrupted {
                            println!("ERR: {:?}", e.kind());
                            break;
                        }
                    }
                }
            }

            stream.flush()?;
        }
    }
}
