use std::io::{ErrorKind, Read, Result, Write};
use std::net::TcpStream;
use std::thread::{self, JoinHandle};

fn write_from_stdin(mut w: TcpStream) -> JoinHandle<()> {
    thread::spawn(move || loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        w.write_all(input.as_bytes()).unwrap();
        w.flush().unwrap();
    })
}

fn read_to_stdout(mut r: TcpStream) -> JoinHandle<()> {
    thread::spawn(move || loop {
        let mut buf = [0u8; 128];
        match r.read(&mut buf) {
            Ok(0) => break,
            Ok(len) => {
                println!(
                    "Received {} bytes from {}: {}",
                    len,
                    r.peer_addr().unwrap(),
                    String::from_utf8_lossy(&buf)
                );
            }
            Err(e) => match e.kind() {
                ErrorKind::Interrupted | ErrorKind::WouldBlock => continue,
                _ => panic!("{}", e),
            },
        }
    })
}

pub fn handle_connection(stream: TcpStream) -> Result<()> {
    let t1 = write_from_stdin(stream.try_clone()?);
    let t2 = read_to_stdout(stream);

    t1.join().unwrap();
    t2.join().unwrap();

    Ok(())
}
