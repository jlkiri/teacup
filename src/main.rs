mod examples;
mod impls;
mod protocol;
mod tcp;
mod udp;

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};

use examples::echo;
use protocol::*;
use structopt::StructOpt;
use tcp::client::TcpClient;
use tcp::server::TcpServer;
use udp::client::UdpClient;
use udp::server::UdpServer;

const UDP: &str = "udp";
const TCP: &str = "tcp";

#[derive(StructOpt, Debug)]
struct ClientOptions {
    addr: String,
    #[structopt(short, long, default_value = TCP, parse(from_str))]
    protocol: Protocol,
    #[structopt(short, long, required_if("protocol", UDP))]
    data: Option<String>,
}

#[derive(StructOpt, Debug)]
struct ServerOptions {
    #[structopt(short, long)]
    port: Option<u16>,
    #[structopt(long)]
    ipv6: bool,
    #[structopt(long, default_value = TCP, parse(from_str))]
    protocol: Protocol,
}

#[derive(StructOpt, Debug)]
enum Examples {
    Echo,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "teacup")]
enum Teacup {
    Listen(ServerOptions),
    Connect(ClientOptions),
    Example(Examples),
}

fn local_addr(port: u16, ipv6: bool) -> SocketAddr {
    if ipv6 {
        return SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 1], port));
    }

    SocketAddr::from(([127, 0, 0, 1], port))
}

fn handle_connection(stream: &mut TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; 32];

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
}

fn main() -> std::io::Result<()> {
    let opt = Teacup::from_args();

    match opt {
        Teacup::Listen(ServerOptions {
            port,
            ipv6,
            protocol,
        }) => match protocol {
            Protocol::Tcp => {
                let addr = local_addr(port.unwrap_or(8888), ipv6);
                let server = TcpServer::bind(addr);
                server.listen(handle_connection)?
            }
            Protocol::Udp => {
                let addr = local_addr(port.unwrap_or(8888), ipv6);
                UdpServer::listen(addr)?
            }
        },
        Teacup::Connect(ClientOptions {
            addr,
            protocol,
            data,
        }) => match protocol {
            Protocol::Tcp => {
                let client = TcpClient::new(addr);
                client.connect()?
            }
            Protocol::Udp => {
                UdpClient::send(addr, data.unwrap_or("Hello!".into()));
            }
        },
        Teacup::Example(ex) => match ex {
            Examples::Echo => echo::run_example()?,
        },
    }

    Ok(())
}
