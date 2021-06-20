use std::io::{ErrorKind, Read, Result, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn read_stdin(stream: Arc<Mutex<TcpStream>>) {
    // let copystream = Arc::clone(&stream);
    thread::spawn(move || loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut w = stream.lock().unwrap();
        w.write_all(input.as_bytes()).unwrap();
        w.flush().unwrap();
    });
}

fn write_stdout(stream: Arc<Mutex<TcpStream>>) -> Result<()> {
    let mut buf = [0u8; 128];

    loop {
        let mut r = stream.lock().unwrap();
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

pub fn handle_connection(strm: TcpStream) -> Result<()> {
    let (tx, rx) = mpsc::channel::<String>();
    let peer_addr = strm.peer_addr()?;

    let stream = Arc::new(Mutex::new(strm));

    stream.lock().unwrap().set_nonblocking(true)?;

    read_stdin(Arc::clone(&stream));
    write_stdout(Arc::clone(&stream));

    Ok(())
}
