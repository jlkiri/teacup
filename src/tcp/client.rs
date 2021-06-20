use crate::connection;
use std::io::ErrorKind;
use std::io::Result;
use std::sync::mpsc;
use std::thread;
use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

pub struct TcpClient;

/* fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    let (tx, rx) = mpsc::channel::<String>();

    stream.set_nonblocking(true)?;

    thread::spawn(move || loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        tx.send(input).unwrap();
    });

    let mut buf = [0u8; 128];

    loop {
        match rx.try_recv() {
            Ok(msg) => {
                stream.write_all(msg.as_bytes())?;
                stream.flush()?;
            }
            Err(..) => (),
        }

        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(len) => {
                println!(
                    "Received {} bytes from {}: {}",
                    len,
                    stream.peer_addr().unwrap(),
                    String::from_utf8_lossy(&buf)
                );
            }
            Err(e) => {
                if e.kind() != ErrorKind::Interrupted {
                    return Err(e);
                }
            }
        }

        stream.flush()?;
    }

    Ok(())
} */

impl TcpClient {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<()> {
        let mut stream = TcpStream::connect(addr)?;
        connection::handle_connection(stream)
    }
}
