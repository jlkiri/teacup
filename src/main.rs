mod client;
mod server;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use client::Client;
use server::Server;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rnc")]
enum Opt {
    Listen {
        #[structopt(short, long)]
        port: Option<u16>,
    },
    Connect {
        #[structopt(short, long)]
        addr: String,
    },
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    match opt {
        Opt::Listen { port } => {
            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port.unwrap_or(3000));
            // let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), port.unwrap_or(3000));
            let server = Server::new(addr);
            server.listen()?;
        }
        Opt::Connect { addr } => {
            let client = Client::new(addr);
            client.connect()?;
        }
    }

    Ok(())
}
