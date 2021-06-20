use std::io::{ErrorKind, Read, Result, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;

pub fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    let (tx, rx) = mpsc::channel::<String>();
    let peer_addr = stream.peer_addr()?;

    stream.set_nonblocking(true)?;

    thread::spawn(move || loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match tx.send(input) {
            Err(..) => {
                 println!("Failed to send a message. Probably lost connection with {}.", peer_addr);
            },
            _ => ()
        }
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
                match e.kind() {
                    ErrorKind::Interrupted | ErrorKind::WouldBlock => continue,
                    _ => return Err(e)
                }
            }
        }
    }

    Ok(())
}
