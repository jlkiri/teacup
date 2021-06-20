use std::io::Result;
use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

pub struct TcpClient;

fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        stream.write_all(input.as_bytes())?;
        stream.flush()?;

        let mut buf = Vec::new();

        stream
            .set_read_timeout(Some(Duration::from_millis(100)))
            .unwrap();

        match stream.read_to_end(&mut buf) {
            Ok(0) => break,
            Ok(len) => {
                println!(
                    "Received {} bytes from {}: {}",
                    len,
                    stream.peer_addr().unwrap(),
                    String::from_utf8(buf).expect("Received invalid utf-8.")
                );
            }
            Err(..) => {}
        }

        stream.flush()?;
    }

    Ok(())
}

impl TcpClient {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<()> {
        let mut stream = TcpStream::connect(addr)?;
        handle_connection(&mut stream)
    }
}
