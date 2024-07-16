use std::net::{TcpStream, Ipv4Addr};
use std::io::Result;

pub fn connect_to_server(ip: Ipv4Addr, port: u16) -> Result<TcpStream> {
    let addr_str = format!("{}:{}", ip, port);
    let stream = TcpStream::connect(addr_str)?;

    Ok(stream)
}