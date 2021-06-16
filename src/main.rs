mod client;
mod server;
mod udp;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use client::Client;
use server::Server;
use structopt::StructOpt;
use udp::{UdpClient, UdpServer};

#[derive(Debug)]
enum Protocol {
    Tcp,
    Udp,
}

impl From<&str> for Protocol {
    fn from(string: &str) -> Self {
        match string {
            "udp" => Self::Udp,
            "tcp" => Self::Tcp,
            _ => panic!("Unknown protocol."),
        }
    }
}

impl From<Protocol> for String {
    fn from(protocol: Protocol) -> String {
        match protocol {
            Protocol::Udp => String::from("udp"),
            Protocol::Tcp => String::from("tcp"),
        }
    }
}

const UDP: &str = "udp";
const TCP: &str = "tcp";

#[derive(StructOpt, Debug)]
struct ClientOptions {
    #[structopt(short, long)]
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
#[structopt(name = "teacup")]
enum Teacup {
    Listen(ServerOptions),
    Connect(ClientOptions),
}

fn local_addr(port: u16, ipv6: bool) -> SocketAddr {
    if ipv6 {
        return SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), port);
    }

    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port)
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
                let server = Server::new(addr);
                server.listen()?
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
                let client = Client::new(addr);
                client.connect()?
            }
            Protocol::Udp => {
                UdpClient::send(addr, data.unwrap_or("Hello!".into()));
            }
        },
    }

    Ok(())
}
