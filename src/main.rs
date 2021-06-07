use std::{io::{Read, Write}, net::*, thread};

const BUFFER_SIZE: usize = 32;

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    loop {
        let mut received: Vec<u8> = vec![];
        let mut buf = [0u8; BUFFER_SIZE];

        let bytes_read = stream.read(&mut buf)?;

        if bytes_read == 0 {
            break;
        }
        
        received.extend_from_slice(&buf[..bytes_read]);

        let string = String::from_utf8(received).expect("Invalid utf-8");

        println!("Message: {}", string);

        stream.write_all(string.as_bytes())?;
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

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
