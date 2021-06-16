mod client;
mod server;
mod udp;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use client::Client;
use server::Server;
use udp::UdpServer;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "teacup")]
enum Opt {
    Listen {
        #[structopt(short, long)]
        port: Option<u16>,
        #[structopt(long)]
        ipv6: bool,
    },
    ListenUdp {
        #[structopt(short, long)]
        port: Option<u16>,
        #[structopt(long)]
        ipv6: bool,
    },
    Connect {
        #[structopt(short, long)]
        addr: String,
    },
}

fn local_addr(port: u16, ipv6: bool) -> SocketAddr {
    if ipv6 {
        return SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), port);
    }

    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port)
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    match opt {
        Opt::Listen { port, ipv6 } => {
            let addr = local_addr(port.unwrap_or(8888), ipv6);
            let server = Server::new(addr);
            server.listen()?
        },
        Opt::ListenUdp { port, ipv6 } => {
            let addr = local_addr(port.unwrap_or(8888), ipv6);
            UdpServer::listen(addr)?
        }
        Opt::Connect { addr } => {
            let client = Client::new(addr);
            client.connect()?
        }
    }

    Ok(())
}
