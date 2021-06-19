use crate::tcp::server::TcpServer;
use std::{
    io::{Read, Write},
    net::*,
};

const BUFFER_SIZE: usize = 32;

pub fn run_example() -> std::io::Result<()> {
    let server = TcpServer::bind(SocketAddr::from(([127, 0, 0, 1], 8888)));

    server.listen(|stream: &mut TcpStream| {
        let mut buf = [0u8; BUFFER_SIZE];

        loop {
            match stream.read(&mut buf) {
                Ok(0) => {
                    break;
                }
                Ok(len) => {
                    let mut received: Vec<u8> = vec![];
                    received.extend_from_slice(&buf[..len]);

                    println!(
                        "Received message: {}",
                        String::from_utf8(received).expect("Invalid utf-8")
                    );

                    stream.write_all(&buf[..len])?;
                }
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::Interrupted {
                        break;
                    }
                }
            }

            stream.flush()?;
        }

        Ok(())
    })?;

    Ok(())
}
