mod client;
mod server;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

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
  }
}


/// "netcat alternative created with Rust"
/* struct Opt {
  #[structopt(short, long)]
  listen: bool,
  #[structopt(short, long)]
  connect: bool,
  #[structopt(required_if("connect", "true"))]
  addr: Option<String>,
  #[structopt(required_if("listen", "true"))]
  port: Option<u16>,
} */

fn main() -> std::io::Result<()> {
  let opt = Opt::from_args();

  match opt {
    Opt::Listen { port } => {
      let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port.unwrap_or(80));
      let server = Server::new(addr);
      server.listen()?;
    },
    Opt::Connect { addr } => {
      let client = Client::new(addr);
      client.connect()?;
    }
  }

  Ok(())
}
