use std::net::{TcpStream};
use std::io::{Read, Write, Result, BufReader, BufRead};


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

pub fn negotiate_algorithms(stream: &mut TcpStream) -> Result<()> {
    // Construct the algorithm negotiation message
    let kex_algorithms = "diffie-hellman-group14-sha256";
    let server_host_key_algorithms = "ssh-rsa";
    let encryption_algorithms = "aes128-ctr";
    let mac_algorithms = "hmac-sha2-256";
    let compression_algorithms = "none";

    let proposal = format!(
        "{},{},{},{},{}\r\n",
        kex_algorithms, server_host_key_algorithms, encryption_algorithms, mac_algorithms, compression_algorithms
    );

    // Send the algorithm proposal
    stream.write_all(proposal.as_bytes())?;
    stream.flush()?;

    // Receive the server's response
    let mut response = vec![0; 1024];
    stream.read(&mut response)?;

    // Print the server's response for now (this is simplified)
    println!("Server Algorithm Response: {:?}", String::from_utf8_lossy(&response));

    Ok(())
}