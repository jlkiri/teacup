#[derive(Debug)]
pub enum Protocol {
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
