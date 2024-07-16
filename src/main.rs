use rssh_client::get_config;
use rssh_client::network::connect_to_server;
use rssh_client::{send_version, receive_version, negotiate_algorithms};

use std::process;

fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let (server_addr, port, user) = match get_config() {
        Ok((addr, port, user)) => (addr, port, user),
        Err(e) => {
            eprintln!("Error: Invalid Arguments, {}", e);
            process::exit(1);
        }
    };

    println!("{} {} {}", server_addr, port, user);

    let mut stream = connect_to_server(server_addr, port)?;

    // Send and receive version strings
    send_version(&mut stream)?;
    let server_version = receive_version(&mut stream)?;

    println!("Server Version: {}", server_version);

    negotiate_algorithms(&mut stream)?;

    Ok(())
}
