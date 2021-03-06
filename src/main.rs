mod conn;
mod examples;
mod protocol;
mod tcp;
mod udp;

use std::io::Result;
use std::net::SocketAddr;

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
    /// Remote address to connect to.
    addr: String,
    #[structopt(short, long, default_value = TCP, parse(from_str))]
    /// Protocol: TCP or UDP.
    protocol: Protocol,
    /// Optionally send data on connection.
    #[structopt(short, long, required_if("protocol", UDP))]
    data: Option<String>,
}

#[derive(StructOpt, Debug)]
struct ServerOptions {
    /// Port to listen on.
    #[structopt(short, long)]
    port: Option<u16>,
    #[structopt(long)]
    /// Tell the server to use IPv6.
    ipv6: bool,
    /// Protocol: TCP or UDP.
    #[structopt(long, default_value = TCP, parse(from_str))]
    protocol: Protocol,
}

#[derive(StructOpt, Debug)]
enum Examples {
    // A simple echo server.
    Echo,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "teacup")]
enum Teacup {
    /// Use as a server.
    Listen(ServerOptions),
    /// Use as a client.
    Connect(ClientOptions),
    /// Launch an example (e.g. echo server).
    Example(Examples),
}

fn local_addr(port: u16, ipv6: bool) -> SocketAddr {
    if ipv6 {
        return SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 1], port));
    }

    SocketAddr::from(([127, 0, 0, 1], port))
}

fn main() -> Result<()> {
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
                server.listen(conn::handle_connection)?;
            }
            Protocol::Udp => {
                let addr = local_addr(port.unwrap_or(8888), ipv6);
                UdpServer::listen(addr)
            }
        },
        Teacup::Connect(ClientOptions {
            addr,
            protocol,
            data,
        }) => match protocol {
            Protocol::Tcp => TcpClient::connect(addr)?,
            Protocol::Udp => UdpClient::send(addr, data.unwrap_or_else(|| "Hello!".into())),
        },
        Teacup::Example(ex) => match ex {
            Examples::Echo => echo::run_example()?,
        },
    }

    Ok(())
}
