use std::time::Duration;
use std::{
    io::{Read, Write},
    net::*,
    thread, vec,
};

const BUFFER_SIZE: usize = 32;

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; BUFFER_SIZE];

    loop {
        println!("Looping.");

        match stream.read(&mut buf) {
            Ok(0) => {
                thread::sleep(Duration::from_millis(100));
            }
            Ok(len) => {
                let mut received: Vec<u8> = vec![];
                received.extend_from_slice(&buf[..len]);

                println!(
                    "Message: {}",
                    String::from_utf8(received).expect("Invalid utf-8")
                );

                stream.write_all(&buf[..len])?;
            }
            Err(e) => {
                if e.kind() != std::io::ErrorKind::Interrupted {
                    println!("{:?}", e.kind());
                    break;
                }
            }
        }

        stream.flush()?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("[::1]:3000")?;

    println!("Listening at {}", listener.local_addr()?);

    loop {
        let (stream, addr) = listener.accept()?;
        println!("Incoming connection from {}", addr);

        handle_connection(stream);

        /* thread::spawn(|| {
            handle_connection(stream);
        }); */
    }
}
