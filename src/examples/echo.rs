use crate::tcp::server::TcpServer;
use std::io::Result;
use std::net::*;

fn echo(mut stream: TcpStream) -> Result<()> {
    let mut w = stream.try_clone()?;
    std::io::copy(&mut stream, &mut w)?;
    Ok(())
}

pub fn run_example() -> std::io::Result<()> {
    let server = TcpServer::bind(("127.0.0.1", 8888));
    server.listen(echo)?;
    Ok(())
}
