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
        #[structopt(short, long)]
        udp: bool,
    },
    Connect {
        #[structopt(short, long)]
        addr: String,
        #[structopt(short, long)]
        udp: bool,
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
        Opt::Listen { port, ipv6, udp: true } => {
          let addr = local_addr(port.unwrap_or(8888), ipv6);
          UdpServer::listen(addr)?
        },
        Opt::Listen { port, ipv6, udp: false } => {
            let addr = local_addr(port.unwrap_or(8888), ipv6);
            let server = Server::new(addr);
            server.listen()?
        },
        Opt::Connect { addr, udp: true } => {
            let client = Client::new(addr);
            client.connect()?
        },
        Opt::Connect { addr, udp: false } => {
          let client = Client::new(addr);
          client.connect()?
      }
    }

    Ok(())
}
