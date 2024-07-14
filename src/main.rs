use rssh_client::get_config;

use std::process;

fn main() {
    let (server_addr, port, user) = match get_config() {
        Ok((addr, port, user)) => (addr, port, user),
        Err(e) => {
            eprintln!("Error: Invalid Arguments, {}", e);
            process::exit(1);
        }
    };

    println!("{} {} {}", server_addr, port, user);
}
