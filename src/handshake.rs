use std::net::{TcpStream};
use std::io::{Write, Result, BufReader, BufRead};


pub fn send_version(stream: &mut TcpStream) -> Result<()> {
    let version = "SSH-2.0-rssh_client_1.0\r\n";
    stream.write_all(version.as_bytes())?;
    stream.flush()?;

    Ok(())
}

pub fn receive_version(stream: &mut TcpStream) -> Result<String> {
    let mut reader = BufReader::new(stream);
    let mut version = String::new();
    reader.read_line(&mut version)?;
    
    Ok(version)
}