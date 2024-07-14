pub mod network;

use std::env;
use std::net::{Ipv4Addr, AddrParseError};
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ArgError{
    InvalidArgs,
    AddrParsing(AddrParseError),
    PortParsing(ParseIntError),
}

impl std::fmt::Display for ArgError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self{
            ArgError::InvalidArgs => {
                write!(f, "Usage: rssh-client <server-address [required]> <port> <user>")
            },
            ArgError::AddrParsing(err) => write!(f, "Invalid IP Address: {}", err),
            ArgError::PortParsing(err) => write!(f, "Invalid Port Number: {}", err),
        }
    }
}

impl std::error::Error for ArgError {}

impl From<AddrParseError> for ArgError {
    fn from(error: AddrParseError) -> Self {
        ArgError::AddrParsing(error)
    }
}

impl From<ParseIntError> for ArgError{
    fn from(error: ParseIntError) -> Self {
        ArgError::PortParsing(error)
    }
}


pub fn get_config() -> Result<(Ipv4Addr, u16, String), ArgError>{
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        return Err(ArgError::InvalidArgs)
    }

    let dest_addr: Ipv4Addr = args[1].parse()?;
    let dest_port: u16 = args[2].parse()?;
    let user = args[3].clone();
    
    Ok((dest_addr, dest_port, user))
}